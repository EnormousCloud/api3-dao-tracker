use crate::nice;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;
use web3::types::U256;

#[derive(Debug, Clone, Deserialize)]
pub struct Prices {
    pub markets: BTreeMap<String, BTreeMap<String, f64>>,
}

pub type CurrentPrices = BTreeMap<String, BTreeMap<String, f64>>;

pub fn coin_price_at(coin: &str, value: U256, decimals: usize, dt: NaiveDateTime) -> Option<f64> {
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(20))
        .timeout_write(Duration::from_secs(5))
        .build();
    let url = format!(
        "https://enormous.cloud/prices/api/{}/at/{}",
        coin,
        dt.format("%Y-%m-%d"),
    );
    let rq = agent.get(&url).set("Content-Type", "application/json");
    // println!("PRICES request={}", &url);
    let response = match rq.call() {
        Ok(x) => x.into_string().unwrap(),
        Err(_e) => {
            // tracing::warn!("PRICES error={:?}", e);
            return None;
        }
    };
    // tracing::debug!("PRICES response={}", response);
    let r = serde_json::from_str::<Prices>(&response).unwrap();

    let eth_prices = r.markets.get(coin).unwrap();
    let market_price = eth_prices.get("usd").unwrap();
    if *market_price < 0.0 {
        return None;
    }
    let price: U256 = nice::shifted_float(*market_price, decimals).unwrap();
    let cost = nice::multiplied(value, price, decimals);
    let out = nice::float(cost, decimals, 2);
    println!(
        "market_price = {} price = {} cost = {} usd = {}",
        *market_price, price, cost, out
    );
    Some(out)
}
