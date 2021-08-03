use client::events::{Api3, VotingAgent};
use client::state::OnChainEvent;
use crc32fast::Hasher;
use futures::StreamExt;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::debug;
use web3::api::Eth;
use web3::transports::{Either, Http, Ipc};
use web3::types::{BlockId, FilterBuilder, Log, H160, H256};
use web3::{Transport, Web3};

pub trait EventHandler {
    fn on(&mut self, entry: OnChainEvent, l: Log) -> ();
}

pub async fn get_transport(source: String) -> Either<Http, Ipc> {
    if source.contains(".ipc") {
        let transport = Ipc::new(source.as_str())
            .await
            .expect("Failed to connect to IPC file");
        debug!("Connected to {:?}", source);
        Either::Right(transport)
    } else {
        let transport = Http::new(source.as_str()).expect("Invalid RPC HTTP endpoint");
        debug!("Connecting to {:?}", source);
        Either::Left(transport)
    }
}

#[derive(Clone, Debug)]
pub struct BlockBatch {
    pub from: u64,
    pub to: u64,
}

pub async fn get_batches<T: Transport>(
    eth: Eth<T>,
    genesis: u64,
    max: Option<u64>,
    batch_size: u64,
) -> Vec<BlockBatch> {
    let max_block: u64 = match max {
        Some(x) => x,
        None => eth
            .block_number()
            .await
            .expect("max block height failure")
            .as_u64(),
    };
    let mut from = genesis;
    let mut res = vec![];
    while from <= max_block {
        let to = if from + batch_size > max_block {
            max_block
        } else {
            from + batch_size - 1
        };
        res.push(BlockBatch { from, to });
        from = from + batch_size
    }
    res
}

#[derive(Debug, Clone)]
pub struct Scanner {
    chain_id: u64,
    cache_dir: String,
    addr_watched: Vec<H160>,
    addr_primary: Vec<H160>,
    addr_secondary: Vec<H160>,
    genesis_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
    blocks_time: BTreeMap<H256, u64>,
}

pub fn blockstime_fn(cache_dir: &str, chain_id: u64) -> String {
    format!("{}/blockstime{}.json", cache_dir, chain_id)
}

pub fn load_blockstime(cache_dir: &str, chain_id: u64) -> BTreeMap<H256, u64> {
    if let Ok(mut f) = File::open(blockstime_fn(cache_dir, chain_id)) {
        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => match serde_json::from_str::<BTreeMap<H256, u64>>(&data) {
                Ok(x) => {
                    tracing::info!("blockstime cache {} records loaded", x.len());
                    x
                }
                Err(e) => {
                    tracing::info!("blockstime JSON parsing failure {}", e);
                    BTreeMap::new()
                }
            },
            Err(_) => BTreeMap::new(),
        }
    } else {
        BTreeMap::new()
    }
}

pub fn save_blockstime(
    cache_dir: &str,
    chain_id: u64,
    blocks_time: &BTreeMap<H256, u64>,
) -> anyhow::Result<()> {
    if cache_dir.len() == 0 {
        return Ok(());
    }
    let f =
        File::create(blockstime_fn(cache_dir, chain_id)).expect("Unable to create blockstime file");
    serde_json::to_writer(&f, blocks_time)?;
    Ok(())
}

impl Scanner {
    pub fn new(
        chain_id: u64,
        cache_dir: &str,
        addr_primary: Vec<H160>,
        addr_secondary: Vec<H160>,
        addr: Vec<H160>,
        genesis_block: u64,
        max_block: Option<u64>,
        batch_size: u64,
    ) -> Self {
        let mut addr_watched: Vec<H160> = addr.clone();
        addr_primary
            .iter()
            .for_each(|x| addr_watched.push(x.clone()));
        addr_secondary
            .iter()
            .for_each(|x| addr_watched.push(x.clone()));

        Self {
            chain_id,
            cache_dir: cache_dir.to_owned(),
            addr_watched,
            addr_primary,
            addr_secondary,
            genesis_block,
            max_block,
            batch_size,
            blocks_time: load_blockstime(&cache_dir, chain_id),
        }
    }
    pub fn agent(&self, address: H160) -> Option<VotingAgent> {
        let mut v: Option<VotingAgent> = None;
        if let Some(_) = self.addr_primary.iter().position(|&r| r == address) {
            v = Some(VotingAgent::Primary);
        }
        if let Some(_) = self.addr_secondary.iter().position(|&r| r == address) {
            v = Some(VotingAgent::Secondary);
        }
        v
    }

    pub fn cache_fn(&self, chain_id: u64, b: &BlockBatch) -> String {
        let mut hasher = Hasher::new();
        self.addr_watched.iter().for_each(|a| {
            hasher.update(format!("{:?}", a).as_bytes());
        });
        let checksum = hasher.finalize();
        format!(
            "{}/chain{}-{}-{}-{}.json",
            self.cache_dir, chain_id, b.from, b.to, checksum
        )
    }

    pub fn has_logs(&self, chain_id: u64, b: &BlockBatch) -> bool {
        if self.cache_dir.len() == 0 {
            return false;
        }
        Path::new(self.cache_fn(chain_id, b).as_str()).exists()
    }

    pub async fn get_logs(&self, chain_id: u64, b: &BlockBatch) -> anyhow::Result<Vec<Log>> {
        let mut f = File::open(self.cache_fn(chain_id, &b)).expect("Unable to open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Reading failure");
        let logs: Vec<Log> = serde_json::from_str(&data).expect("JSON parsing failure");
        Ok(logs)
    }

    pub async fn save_logs(
        &self,
        chain_id: u64,
        b: &BlockBatch,
        logs: &Vec<Log>,
    ) -> anyhow::Result<()> {
        if self.cache_dir.len() == 0 {
            return Ok(());
        }
        let f = File::create(self.cache_fn(chain_id, &b)).expect("Unable to create file");
        serde_json::to_writer(&f, logs)?;
        Ok(())
    }

    pub async fn scan<T>(
        &mut self,
        web3: &Web3<T>,
        handler: &mut impl EventHandler,
    ) -> anyhow::Result<u64>
    where
        T: Transport,
    {
        let chain_id = self.chain_id;
        let mut last_block = self.genesis_block;
        for b in get_batches(
            web3.eth(),
            self.genesis_block,
            self.max_block,
            self.batch_size,
        )
        .await
        {
            let start = std::time::Instant::now();
            let mut method = "".to_owned();
            let _ = method; // dummy warning workaround
            let logs: Vec<Log> = if self.has_logs(chain_id, &b) {
                let logs = self.get_logs(chain_id, &b).await?;
                method = format!(
                    "cached {}..{}/{} in {:?}",
                    b.from,
                    b.to,
                    chain_id,
                    start.elapsed()
                );
                logs
            } else {
                let filter = FilterBuilder::default()
                    .from_block(b.from.into())
                    .to_block(b.to.into())
                    .address(self.addr_watched.clone())
                    .build();
                let logs: Vec<Log> = web3.eth().logs(filter).await?;
                self.save_logs(chain_id, &b, &logs).await?;
                method = format!(
                    "scanned {}..{}/{} in {:?}",
                    b.from,
                    b.to,
                    chain_id,
                    start.elapsed()
                );
                logs
            };

            // let start = std::time::Instant::now();
            let mut blocktime_dur = std::time::Duration::from_nanos(0);
            let mut handler_dur = std::time::Duration::from_nanos(0);
            for l in &logs {
                if let Ok(entry) = Api3::from_log(self.agent(l.address), &l) {
                    let blockstart = std::time::Instant::now();
                    let tmkey: H256 = l.block_hash.unwrap();
                    let ts: u64 = if self.blocks_time.contains_key(&tmkey) {
                        match self.blocks_time.get(&tmkey) {
                            Some(x) => x.clone(),
                            None => web3
                                .eth()
                                .block(BlockId::Hash(l.block_hash.unwrap()))
                                .await
                                .expect("block failure")
                                .expect("block timestamp failure")
                                .timestamp
                                .as_u64(),
                        }
                    } else {
                        web3.eth()
                            .block(BlockId::Hash(l.block_hash.unwrap()))
                            .await
                            .expect("block failure")
                            .expect("block timestamp failure")
                            .timestamp
                            .as_u64()
                    };
                    self.blocks_time.insert(tmkey, ts);

                    blocktime_dur += blockstart.elapsed();

                    let handlerstart = std::time::Instant::now();
                    handler.on(
                        OnChainEvent {
                            block_number: l.block_number.unwrap().as_u64(),
                            tx: l.transaction_hash.unwrap(),
                            log_index: l.log_index.unwrap().as_u64(),
                            entry,
                            tm: ts,
                        },
                        l.clone(),
                    );
                    handler_dur += handlerstart.elapsed();
                }
            }
            save_blockstime(&self.cache_dir, chain_id, &self.blocks_time)?;
            tracing::info!(
                "{} events, took {:?}, blocks {:?} ({})",
                logs.len(),
                handler_dur,
                blocktime_dur,
                method,
            );
            last_block = b.to;
        }
        Ok(last_block)
    }

    // continuously watch incoming blocks
    pub async fn watch_ipc(
        &mut self,
        web3: &Web3<Ipc>,
        from_block: u64,
        handler_mux: Arc<Mutex<impl EventHandler>>,
    ) -> anyhow::Result<()> {
        tracing::info!("listening to blocks from {} in real-time", from_block);
        let filter = FilterBuilder::default()
            .from_block(from_block.into())
            .address(self.addr_watched.clone())
            .build();
        let filter = web3.eth_filter().create_logs_filter(filter).await.unwrap();
        let logs_stream = filter.stream(Duration::from_secs(10));
        futures::pin_mut!(logs_stream);
        loop {
            tracing::info!("waiting for entries");
            let l: Log = logs_stream.next().await.unwrap().unwrap();
            if let Ok(entry) = Api3::from_log(self.agent(l.address), &l) {
                let tmkey: H256 = l.block_hash.unwrap();
                let tm: u64 = web3
                    .eth()
                    .block(BlockId::Hash(l.block_hash.unwrap()))
                    .await
                    .expect("block failure")
                    .expect("block timestamp failure")
                    .timestamp
                    .as_u64();

                handler_mux.lock().unwrap().on(
                    OnChainEvent {
                        block_number: l.block_number.unwrap().as_u64(),
                        tx: l.transaction_hash.unwrap(),
                        log_index: l.log_index.unwrap().as_u64(),
                        entry,
                        tm,
                    },
                    l,
                );
                self.blocks_time.insert(tmkey, tm);
                save_blockstime(&self.cache_dir, self.chain_id, &self.blocks_time)?;

                // match &entry {
                //     Api3::StartVote{ agent, vote_id, creator: _, metadata: _ } => {
                //         let key = voting_to_u64(agent, vote_id.as_u64());
                //         if let Some(v) = self.votings.get_mut(&key) {
                //             let static_data = conv.get_voting_static_data(v.primary, v.creator, v.vote_id).await;
                //             println!("voting_static_data = {:?}", static_data);
                //             if let Some(data) = static_data  {
                //                 v.votes_total = data.voting_power; // adjust with precise #
                //                 v.static_data = static_data;
                //             }
                //         }
                //     },
                //     _ => {},
                // };
            }
        }
    }
}
