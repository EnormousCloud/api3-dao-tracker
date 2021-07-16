use client::events::{Api3, VotingAgent};
use client::state::OnChainEvent;
use crc32fast::Hasher;
use futures::StreamExt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::debug;
use web3::api::Eth;
use web3::transports::{Either, Http, Ipc};
use web3::types::{BlockId, FilterBuilder, Log, H160, U256};
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
    batch_size: u64,
) -> Vec<BlockBatch> {
    let max_block: u64 = eth
        .block_number()
        .await
        .expect("max block height failure")
        .as_u64();
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
    cache_dir: String,
    addr_watched: Vec<H160>,
    addr_primary: Vec<H160>,
    addr_secondary: Vec<H160>,
    genesis_block: u64,
    batch_size: u64,
}

impl Scanner {
    pub fn new(
        cache_dir: &str,
        addr_primary: Vec<H160>,
        addr_secondary: Vec<H160>,
        addr: Vec<H160>,
        genesis_block: u64,
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
            cache_dir: cache_dir.to_owned(),
            addr_watched,
            addr_primary,
            addr_secondary,
            genesis_block,
            batch_size,
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
        &self,
        web3: &Web3<T>,
        handler: &mut impl EventHandler,
    ) -> anyhow::Result<u64>
    where
        T: Transport,
    {
        let chain_id = web3.eth().chain_id().await?.as_u64();
        let mut last_block = self.genesis_block;
        for b in get_batches(web3.eth(), self.genesis_block, self.batch_size).await {
            let logs: Vec<Log> = if self.has_logs(chain_id, &b) {
                tracing::debug!(
                    "pulling cached blocks {}..{} chain_id {}",
                    b.from,
                    b.to,
                    chain_id
                );
                self.get_logs(chain_id, &b).await?
            } else {
                tracing::debug!("scanning blocks {}..{}", b.from, b.to);
                let filter = FilterBuilder::default()
                    .from_block(b.from.into())
                    .to_block(b.to.into())
                    .address(self.addr_watched.clone())
                    .build();
                let logs: Vec<Log> = web3.eth().logs(filter).await?;
                self.save_logs(chain_id, &b, &logs).await?;
                logs
            };
            for l in logs {
                if let Ok(entry) = Api3::from_log(self.agent(l.address), &l) {
                    let ts: U256 = web3
                        .eth()
                        .block(BlockId::Hash(l.block_hash.unwrap()))
                        .await
                        .expect("block failure")
                        .expect("block timestamp failure")
                        .timestamp;

                    handler.on(
                        OnChainEvent {
                            block_number: l.block_number.unwrap().as_u64(),
                            tx: l.transaction_hash.unwrap(),
                            log_index: l.log_index.unwrap().as_u64(),
                            entry,
                            tm: ts.as_u64(),
                        },
                        l,
                    );
                }
            }
            last_block = b.to;
        }
        Ok(last_block)
    }

    // continuously watch incoming blocks
    pub async fn watch_ipc(
        &self,
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
                let ts: U256 = web3
                    .eth()
                    .block(BlockId::Hash(l.block_hash.unwrap()))
                    .await
                    .expect("block failure")
                    .expect("block timestamp failure")
                    .timestamp;

                handler_mux.lock().unwrap().on(
                    OnChainEvent {
                        block_number: l.block_number.unwrap().as_u64(),
                        tx: l.transaction_hash.unwrap(),
                        log_index: l.log_index.unwrap().as_u64(),
                        entry,
                        tm: ts.as_u64(),
                    },
                    l,
                );
            }
        }
    }
}
