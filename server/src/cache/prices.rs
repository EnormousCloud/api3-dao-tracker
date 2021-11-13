use client::fees::TxFee;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use web3::types::H256;

fn filename(cache_dir: &str, chain_id: u64) -> String {
    format!("{}/prices{}.json", cache_dir, chain_id)
}

pub fn load(cache_dir: &str, chain_id: u64) -> BTreeMap<H256, TxFee> {
    if let Ok(mut f) = File::open(filename(cache_dir, chain_id)) {
        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => match serde_json::from_str::<BTreeMap<H256, TxFee>>(&data) {
                Ok(x) => {
                    tracing::info!("prices cache {} records loaded", x.len());
                    x
                }
                Err(e) => {
                    tracing::info!("prices JSON parsing failure {}", e);
                    BTreeMap::new()
                }
            },
            Err(_) => BTreeMap::new(),
        }
    } else {
        BTreeMap::new()
    }
}

pub fn save(cache_dir: &str, chain_id: u64, values: &BTreeMap<H256, TxFee>) -> anyhow::Result<()> {
    if cache_dir.len() == 0 || values.len() == 0 {
        return Ok(());
    }
    let f =
        File::create(filename(cache_dir, chain_id)).expect("Unable to create prices cache file");
    serde_json::to_writer(&f, values)?;
    Ok(())
}
