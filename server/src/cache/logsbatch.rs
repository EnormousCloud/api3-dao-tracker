use crc32fast::Hasher;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use web3::types::{Log, H160};

#[derive(Clone, Debug)]
pub struct BlockBatch {
    pub from: u64,
    pub to: u64,
}

pub fn checksum(addr_watched: &Vec<H160>) -> u32 {
    let mut hasher = Hasher::new();
    addr_watched.iter().for_each(|a| {
        hasher.update(format!("{:?}", a).as_bytes());
    });
    hasher.finalize()
}

pub fn filename(cache_dir: &str, chain_id: u64, checksum: u32, b: &BlockBatch) -> String {
    format!(
        "{}/chain{}-{}-{}-{}.json",
        cache_dir, chain_id, b.from, b.to, checksum,
    )
}

pub fn exists(cache_dir: &str, chain_id: u64, checksum: u32, b: &BlockBatch) -> bool {
    if cache_dir.len() == 0 {
        return false;
    }
    Path::new(filename(cache_dir, chain_id, checksum, b).as_str()).exists()
}

pub async fn load(
    cache_dir: &str,
    chain_id: u64,
    checksum: u32,
    b: &BlockBatch,
) -> anyhow::Result<Vec<Log>> {
    let mut f =
        File::open(filename(cache_dir, chain_id, checksum, &b)).expect("Unable to open file");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Reading failure");
    let logs: Vec<Log> = serde_json::from_str(&data).expect("JSON parsing failure");
    Ok(logs)
}

pub async fn save(
    cache_dir: &str,
    chain_id: u64,
    checksum: u32,
    b: &BlockBatch,
    logs: &Vec<Log>,
) -> anyhow::Result<()> {
    if cache_dir.len() == 0 {
        return Ok(());
    }
    let f =
        File::create(filename(cache_dir, chain_id, checksum, &b)).expect("Unable to create file");
    serde_json::to_writer(&f, logs)?;
    Ok(())
}
