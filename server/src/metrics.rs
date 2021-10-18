use client::state::AppState;
use lazy_static::lazy_static;
use prometheus::{opts, register_int_gauge}; //, register_gauge, register_histogram_vec}
use prometheus::{Encoder, IntGauge, Registry, TextEncoder};
use std::collections::HashMap;

lazy_static! {
    pub static ref CHAIN_ID_GAUGE: IntGauge =
        register_int_gauge!(opts!("chain_id", "Chain ID that is being syncing",)).unwrap();
    pub static ref WATCHING: IntGauge =
        register_int_gauge!(opts!("watching", "Whether the chain is being watched",)).unwrap();
    pub static ref BLOCK_START_GAUGE: IntGauge =
        register_int_gauge!(opts!("block_start", "Number of block that is being synced",)).unwrap();
    pub static ref BLOCK_END_GAUGE: IntGauge =
        register_int_gauge!(opts!("block_end", "Number of block that is being synced",)).unwrap();
}

lazy_static! {
    pub static ref NUM_ADDRESSES: IntGauge =
        register_int_gauge!(opts!("addresses", "Number of addresses",)).unwrap();
}

pub fn handler(state: &AppState) -> String {
    let encoder = TextEncoder::new();
    let labels = HashMap::new();
    let sr = Registry::new_custom(Some("dao".to_string()), Some(labels)).unwrap();
    sr.register(Box::new(NUM_ADDRESSES.clone())).unwrap();
    NUM_ADDRESSES.set(state.wallets.len() as i64);

    sr.register(Box::new(CHAIN_ID_GAUGE.clone())).unwrap();
    sr.register(Box::new(WATCHING.clone())).unwrap();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&sr.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}

pub fn syncing() -> String {
    let encoder = TextEncoder::new();
    let labels = HashMap::new();
    let sr = Registry::new_custom(Some("sync".to_string()), Some(labels)).unwrap();
    sr.register(Box::new(CHAIN_ID_GAUGE.clone())).unwrap();
    sr.register(Box::new(WATCHING.clone())).unwrap();
    sr.register(Box::new(BLOCK_START_GAUGE.clone())).unwrap();
    sr.register(Box::new(BLOCK_END_GAUGE.clone())).unwrap();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&sr.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
