use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use web3::types::Log;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archive {
    pub start_block: u64,
    pub end_block: u64,
    pub logs: Vec<Log>,
}

fn filename(cache_dir: &str, chain_id: u64, dt: SystemTime) -> String {
    let d: DateTime<Utc> = dt.into();
    let dts = d.format("%Y%m%d%H%M%s").to_string();
    format!("{}/snapshot{}/{}.json", cache_dir, chain_id, dts)
}

fn latest(cache_dir: &str, chain_id: u64) -> Option<String> {
    // getting the latest file in the folder
    let dir_path = format!("{}/snapshot{}", cache_dir, chain_id);
    let iter = match std::fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let mut result: Option<String> = None;
    for entry in iter {
        let e = match entry {
            Ok(x) => x.path().display().to_string(),
            Err(_) => continue,
        };
        if e.ends_with(".json") {
            match &result {
                Some(v) => {
                    if let Ordering::Less = v.cmp(&e) {
                        result = Some(e)
                    }
                }
                None => result = Some(e),
            };
        }
    }
    result
}

pub fn load(cache_dir: &str, chain_id: u64) -> Option<Archive> {
    // find the latest snapshot in the snapshots folder
    let path = match latest(cache_dir, chain_id) {
        Some(x) => x,
        None => {
            tracing::info!("no snapshots for chain {}", chain_id);
            return None;
        }
    };

    if let Ok(mut f) = File::open(path) {
        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => match serde_json::from_str::<Archive>(&data) {
                Ok(x) => {
                    tracing::info!(
                        "snapshot cache: {} records loaded {}..{}",
                        x.logs.len(),
                        x.start_block,
                        x.end_block
                    );
                    return Some(x);
                }
                Err(e) => {
                    tracing::info!("snapshots JSON parsing failure {}", e);
                    return None;
                }
            },
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn save(cache_dir: &str, chain_id: u64, archive: &Archive) -> anyhow::Result<()> {
    if cache_dir.len() == 0 || archive.logs.len() == 0 {
        return Ok(());
    }
    let now = SystemTime::now();
    let fln = filename(cache_dir, chain_id, now);
    println!("saving snapshot {}", fln);
    if let Err(e) = std::fs::create_dir_all(match std::path::Path::new(&fln).parent() {
        Some(x) => x,
        None => return Ok(()),
    }) {
        println!("snapshot folder failure {:?}", e);
        return Ok(());
    }

    let f = File::create(fln).expect("Unable to create snapshot file");
    serde_json::to_writer(&f, archive)?;
    Ok(())
}
