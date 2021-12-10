use crate::reader;
use client::events::Api3;
use client::state::OnChainEvent;
use std::collections::BTreeMap;
use web3::types::H256;

pub struct Unknown {
    pub unknown_topics: BTreeMap<H256, H256>,
}
impl Unknown {
    pub fn new() -> Self {
        Self {
            unknown_topics: BTreeMap::new(),
        }
    }
    pub fn done(&self) -> () {
        if self.unknown_topics.len() > 0 {
            for (topic, t) in &self.unknown_topics {
                tracing::warn!("unknown topic {:?} in {:?}", topic, t);
            }
        }
    }
}

impl reader::EventHandler for Unknown {
    fn on(&mut self, e: OnChainEvent, l: web3::types::Log) -> () {
        if let Api3::Unknown = e.entry {
            if !self.unknown_topics.contains_key(&l.topics[0]) {
                self.unknown_topics
                    .insert(l.topics[0], l.transaction_hash.unwrap());
            }
            tracing::warn!("{:?} {:?}", l.transaction_hash.unwrap(), e.entry);
        }
    }
}

pub struct Events {}

impl Events {
    pub fn new() -> Self {
        Self {}
    }
}

impl reader::EventHandler for Events {
    fn on(&mut self, entry: OnChainEvent, _: web3::types::Log) -> () {
        println!("{}", serde_json::to_string(&entry).unwrap());
    }
}

pub struct SnapshotBuilder {
    pub cache_dir: String,
    pub chain_id: u64,
    pub logs: Vec<web3::types::Log>,
}
impl SnapshotBuilder {
    pub fn new(cache_dir: &str, chain_id: u64) -> Self {
        Self {
            cache_dir: cache_dir.to_string(),
            chain_id,
            logs: vec![],
        }
    }
    pub fn done(&self) -> () {
        let _ = crate::cache::snapshot::save(&self.cache_dir, self.chain_id, &self.logs);
    }
}

impl reader::EventHandler for SnapshotBuilder {
    fn on(&mut self, _e: OnChainEvent, l: web3::types::Log) -> () {
        self.logs.push(l);
    }
}
