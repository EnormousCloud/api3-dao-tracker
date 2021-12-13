use crate::cache::blockstime;
use crate::cache::logsbatch::{self, BlockBatch};
use crate::cache::prices;
use crate::web3sync::EthClient;
use chrono::NaiveDateTime;
use client::events::{Api3, VotingAgent};
use client::fees::TxFee;
use client::state::OnChainEvent;
use std::collections::BTreeMap;
use std::sync::Mutex;
use web3::api::Eth;
use web3::types::{BlockId, FilterBuilder, Log, H160, H256};
use web3::{Transport, Web3};

pub trait EventHandler {
    fn on(&mut self, entry: OnChainEvent, l: Log) -> ();
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
    rpc_endpoint: String,
    chain_id: u64,
    cache_dir: String,
    addr_watched: Vec<H160>,
    addr_primary: Vec<H160>,
    addr_secondary: Vec<H160>,
    genesis_block: u64,
    max_block: Option<u64>,
    batch_size: u64,
    blocks_time: BTreeMap<H256, u64>,
    fees: BTreeMap<H256, TxFee>,
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
        rpc_endpoint: &str,
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
            rpc_endpoint: rpc_endpoint.to_string(),
            addr_watched,
            addr_primary,
            addr_secondary,
            genesis_block,
            max_block,
            batch_size,
            blocks_time: blockstime::load(&cache_dir, chain_id),
            fees: prices::load(&cache_dir, chain_id),
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

    async fn handle_logs<T>(
        &mut self,
        web3: &Web3<T>,
        method: &str,
        handler: &mut impl EventHandler,
        w3client: &EthClient,
        logs: &Vec<Log>,
    ) -> anyhow::Result<u64>
    where
        T: Transport,
    {
        // let start = std::time::Instant::now();
        let mut blocktime_dur = std::time::Duration::from_nanos(0);
        let mut prices_dur = std::time::Duration::from_nanos(0);
        let mut handler_dur = std::time::Duration::from_nanos(0);
        for l in logs {
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

                let txkey: H256 = l.transaction_hash.unwrap();
                let pricesstart = std::time::Instant::now();
                let dt = NaiveDateTime::from_timestamp(ts as i64, 0);

                let fees = match self.fees.get(&txkey) {
                    Some(x) => x.from(dt),
                    None => {
                        let txfee = w3client.fees(txkey, dt).unwrap();
                        prices::save(&self.cache_dir, self.chain_id, &self.fees)?;
                        txfee
                    }
                };
                self.fees.insert(txkey, fees.clone());
                prices_dur += pricesstart.elapsed();

                let handlerstart = std::time::Instant::now();
                handler.on(
                    OnChainEvent {
                        block_number: l.block_number.unwrap().as_u64(),
                        tx: txkey,
                        log_index: l.log_index.unwrap().as_u64(),
                        entry,
                        tm: ts,
                        fees,
                    },
                    l.clone(),
                );
                handler_dur += handlerstart.elapsed();
            }
        }

        tracing::info!(
            "{} events, took {:?}, blocks {:?}, prices {:?} ({})",
            logs.len(),
            handler_dur,
            blocktime_dur,
            prices_dur,
            method,
        );
        Ok(0)
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
        let cache_dir = self.cache_dir.clone();
        let checksum = logsbatch::checksum(&self.addr_watched);
        crate::metrics::CHAIN_ID_GAUGE.set(chain_id as i64);
        let w3client = EthClient::new(&self.rpc_endpoint);
        let mut last_block = self.genesis_block;

        if let Some(archive) = crate::cache::snapshot::load(&cache_dir, chain_id) {
            crate::metrics::BLOCK_START_GAUGE.set(archive.start_block as i64);
            crate::metrics::BLOCK_END_GAUGE.set(archive.end_block as i64);
            let start = std::time::Instant::now();
            let method = format!(
                "snapshot {}..{}/{}",
                archive.start_block, archive.end_block, chain_id
            );
            if let Err(e) = self
                .handle_logs(web3, &method, handler, &w3client, &archive.logs)
                .await
            {
                tracing::error!("error {}", e);
            }
            blockstime::save(&self.cache_dir, chain_id, &self.blocks_time)?;
            tracing::info!("{} restored in {:?}", method, start.elapsed());
            crate::metrics::BLOCK_START_GAUGE.set(0);
            crate::metrics::BLOCK_END_GAUGE.set(0);
            return Ok(archive.end_block);
        }

        for b in get_batches(
            web3.eth(),
            self.genesis_block,
            self.max_block,
            self.batch_size,
        )
        .await
        {
            crate::metrics::BLOCK_START_GAUGE.set(b.from as i64);
            crate::metrics::BLOCK_END_GAUGE.set(b.to as i64);
            let start = std::time::Instant::now();
            let mut method = "".to_owned();
            let _ = method; // dummy warning workaround
            let logs: Vec<Log> = if logsbatch::exists(&cache_dir, chain_id, checksum, &b) {
                let logs = logsbatch::load(&cache_dir, chain_id, checksum, &b).await?;
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
                logsbatch::save(&cache_dir, chain_id, checksum, &b, &logs).await?;
                method = format!(
                    "scanned {}..{}/{} in {:?}",
                    b.from,
                    b.to,
                    chain_id,
                    start.elapsed()
                );
                last_block = b.to;
                logs
            };

            if let Err(e) = self
                .handle_logs(web3, &method, handler, &w3client, &logs)
                .await
            {
                tracing::error!("error {}", e);
            }
            blockstime::save(&self.cache_dir, chain_id, &self.blocks_time)?;
        }
        crate::metrics::BLOCK_START_GAUGE.set(0);
        crate::metrics::BLOCK_END_GAUGE.set(0);
        Ok(last_block)
    }

    // continuously watch incoming blocks.
    pub fn watch_http(
        &mut self,
        endpoint_addr: &str,
        from_block: u64,
        handler_mux: &Mutex<impl EventHandler>,
    ) -> anyhow::Result<()> {
        tracing::info!(
            "listening to blocks from {} in real-time {}",
            from_block,
            endpoint_addr
        );
        let w3client = EthClient::new(endpoint_addr);
        let filter = FilterBuilder::default()
            .from_block(from_block.into())
            .address(self.addr_watched.clone())
            .build();
        let filter_id = w3client.new_filter(&filter).expect("new_filter error");
        crate::metrics::WATCHING.set(1);
        // let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));
        loop {
            tracing::info!("waiting {:?}", std::time::Duration::from_secs(20));
            std::thread::sleep(std::time::Duration::from_secs(20));
            tracing::info!("filter_id {:?} from block {}", filter_id, from_block);
            match w3client.filter_changes(filter_id) {
                Ok(logs) => {
                    for l in logs {
                        if let Ok(entry) = Api3::from_log(self.agent(l.address), &l) {
                            let bhash: H256 = l.block_hash.expect("block hash");
                            let block_number = l.block_number.expect("block number").as_u64();
                            let tx = l.transaction_hash.expect("tx hash");
                            let log_index = l.log_index.expect("log_index").as_u64();
                            let tm = match self.blocks_time.get(&bhash) {
                                Some(x) => *x,
                                None => {
                                    let tm = w3client
                                        .block(bhash)
                                        .expect("block timestamp failure")
                                        .timestamp
                                        .as_u64();
                                    self.blocks_time.insert(bhash, tm);
                                    blockstime::save(
                                        &self.cache_dir,
                                        self.chain_id,
                                        &self.blocks_time,
                                    )?;
                                    tm
                                }
                            };
                            let dt = NaiveDateTime::from_timestamp(tm as i64, 0);
                            let fees = match self.fees.get(&bhash) {
                                Some(x) => (*x).clone(),
                                None => {
                                    let txfee = w3client.fees(tx, dt).unwrap();
                                    self.fees.insert(bhash, txfee.clone());
                                    prices::save(&self.cache_dir, self.chain_id, &self.fees)?;
                                    txfee
                                }
                            };
                            handler_mux.lock().expect("unlock event handler mutex").on(
                                OnChainEvent {
                                    block_number,
                                    tx,
                                    log_index,
                                    entry,
                                    tm,
                                    fees,
                                },
                                l,
                            );
                        }
                    }
                }
                Err(err) => {
                    tracing::error!("filter_id error {:?}", err);
                    return Err(err);
                }
            };
        }
    }
}
