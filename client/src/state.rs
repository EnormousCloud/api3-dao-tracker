use serde::{Deserialize, Serialize};
use web3::types::{H160, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voting {
    pub id: u64,
    pub title: String,
    pub command: String,
    pub primary: bool,
    // TODO: events under this voting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: H160,
    pub deposited: U256,
    pub withdrawn: U256,
    pub staked: U256,
    pub shares: U256,
    pub delegated_to: Option<H160>,
    // TODO: events under this wallet
}
