use crate::reader;
use client::events;
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
    fn on(&mut self, entry: events::Api3, l: web3::types::Log) -> () {
        if let events::Api3::Unknown = entry {
            if !self.unknown_topics.contains_key(&l.topics[0]) {
                self.unknown_topics
                    .insert(l.topics[0], l.transaction_hash.unwrap());
            }
            tracing::warn!("{:?} {:?}", l.transaction_hash.unwrap(), entry);
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
    fn on(&mut self, entry: events::Api3, l: web3::types::Log) -> () {
        let tx = l.transaction_hash.unwrap();
        // serde_json::to_string(&entry).unwrap()
        println!("{:?} {:?}", tx, entry);
    }
}
