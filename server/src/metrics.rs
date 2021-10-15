use client::state::AppState;
use lazy_static::lazy_static;
use prometheus::{labels, opts, register_int_gauge}; //, register_gauge, register_histogram_vec}
use prometheus::{Encoder, IntGauge, TextEncoder};

lazy_static! {
    pub static ref SYNC_CHAIN_ID_GAUGE: IntGauge = register_int_gauge!(opts!(
        "sync_chain_id",
        "Chain ID that is being syncing",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    pub static ref SYNC_BLOCK_START_GAUGE: IntGauge = register_int_gauge!(opts!(
        "sync_block_start",
        "Number of block that is being synced",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    pub static ref SYNC_BLOCK_END_GAUGE: IntGauge = register_int_gauge!(opts!(
        "sync_block_end",
        "Number of block that is being synced",
        labels! {"handler" => "all",}
    ))
    .unwrap();
}

pub fn handler(_state: &AppState) -> String {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}

pub fn syncing() -> String {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
