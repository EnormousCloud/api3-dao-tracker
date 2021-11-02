use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use web3::types::H256;

fn filename(cache_dir: &str, chain_id: u64) -> String {
    format!("{}/blockstime{}.json", cache_dir, chain_id)
}

pub fn load(cache_dir: &str, chain_id: u64) -> BTreeMap<H256, u64> {
    if let Ok(mut f) = File::open(filename(cache_dir, chain_id)) {
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

pub fn save(
    cache_dir: &str,
    chain_id: u64,
    blocks_time: &BTreeMap<H256, u64>,
) -> anyhow::Result<()> {
    if cache_dir.len() == 0 || blocks_time.len() == 0 {
        return Ok(());
    }
    let f = File::create(filename(cache_dir, chain_id)).expect("Unable to create blockstime file");
    serde_json::to_writer(&f, blocks_time)?;
    Ok(())
}
