use client::state::AppState;
use lazy_static::lazy_static;
use prometheus::{labels, opts, register_counter}; //, register_gauge, register_histogram_vec}
use prometheus::{Counter, Encoder, TextEncoder};

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
}

pub fn handler(_state: &AppState) -> String {
    let encoder = TextEncoder::new();
    HTTP_COUNTER.inc();

    let metric_families = prometheus::gather();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}

pub fn syncing() -> String {
    let encoder = TextEncoder::new();
    HTTP_COUNTER.inc();

    let metric_families = prometheus::gather();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
