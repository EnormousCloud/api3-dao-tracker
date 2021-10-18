use client::state::AppState;
use lazy_static::lazy_static;
use prometheus::{opts, register_gauge, register_int_gauge};
use prometheus::{Encoder, Gauge, IntGauge, Registry, TextEncoder};
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
        register_int_gauge!(opts!("addresses", "Number of addresses in DAO",)).unwrap();
    pub static ref NUM_VOTINGS: IntGauge =
        register_int_gauge!(opts!("votings", "Number of votings in DAO",)).unwrap();
    pub static ref EPOCHS: IntGauge =
        register_int_gauge!(opts!("epochs", "Number of epochs",)).unwrap();
    pub static ref EPOCH_INDEX: IntGauge =
        register_int_gauge!(opts!("epoch_index", "Index of the current epoch",)).unwrap();
    pub static ref APR: Gauge =
        register_gauge!(opts!("apr", "Index of the current epoch",)).unwrap();
    pub static ref LAST_BLOCK: IntGauge =
        register_int_gauge!(opts!("last_block", "Index of the last block",)).unwrap();
}

pub fn handler(state: &AppState) -> String {
    let encoder = TextEncoder::new();
    let labels = HashMap::new();
    let sr = Registry::new_custom(Some("dao".to_string()), Some(labels)).unwrap();
    sr.register(Box::new(NUM_ADDRESSES.clone())).unwrap();
    sr.register(Box::new(NUM_VOTINGS.clone())).unwrap();
    sr.register(Box::new(CHAIN_ID_GAUGE.clone())).unwrap();
    sr.register(Box::new(WATCHING.clone())).unwrap();
    sr.register(Box::new(EPOCHS.clone())).unwrap();
    sr.register(Box::new(EPOCH_INDEX.clone())).unwrap();
    sr.register(Box::new(APR.clone())).unwrap();
    sr.register(Box::new(LAST_BLOCK.clone())).unwrap();

    NUM_ADDRESSES.set(state.wallets.len() as i64);
    NUM_VOTINGS.set(state.votings.len() as i64);
    EPOCHS.set(state.epochs.len() as i64);
    EPOCH_INDEX.set(state.epoch_index as i64);
    APR.set(state.apr);
    LAST_BLOCK.set(state.last_block as i64);

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
