use std::str::FromStr;
use std::sync::{Arc, Mutex};
use warp::Filter;
use web3::types::H160;

pub fn routes(
    state: Arc<Mutex<crate::State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let wallets = warp::path!("wallets").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            let mut out: Vec<String> = vec![];
            for (k, _) in &state.wallets {
                out.push(format!("{:?}", k));
            }
            format!("# API3 DAO {} Wallets\n{}", out.len(), out.join("\n"))
        }
    });
    let votings = warp::path!("votings").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            let mut out: Vec<String> = vec![];
            for (k, v) in &state.votings {
                out.push(format!("{:?} {:?}", k, v));
            }
            format!("# API3 DAO {} Votings\n{}", out.len(), out.join("\n"))
        }
    });
    let wallet = warp::path!("wallets" / String).map({
        let state_rc = state.clone();
        move |id: String| {
            let state = state_rc.lock().unwrap();
            if let Ok(addr) = H160::from_str(id.as_str()) {
                format!("# API3 DAO Wallet\n{:?}", state.wallets.get(&addr))
            } else {
                format!("Invalid wallet")
            }
        }
    });
    let voting = warp::path!("votings" / u64).map({
        let state_rc = state.clone();
        move |id: u64| {
            let state = state_rc.lock().unwrap();
            format!("# API3 DAO Votings\n{:?}", state.votings.get(&id))
        }
    });

    let ping = warp::path::end().map(|| format!("# API3 DAO Tracker"));
    ping.or(wallet).or(wallets).or(voting).or(votings)
}
