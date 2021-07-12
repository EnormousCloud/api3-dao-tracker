use crate::events::Api3;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use web3::types::{H160, H256, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnChainEvent {
    pub entry: Api3,
    pub tm: u64,
    pub block_number: u64,
    pub tx: H256,
    pub log_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voting {
    pub id: String,
    pub title: String,
    pub command: String,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: H160,
    pub ens: Option<String>,
    pub deposited: U256,
    pub withdrawn: U256,
    pub staked: U256,
    pub shares: U256,
    pub delegated_to: Option<H160>,
    // TODO: time of first activity
    // TODO: time of the last activity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// the block of the last event
    pub last_block: u64,
    /// log of events, grouped by votings
    pub votings: BTreeMap<u64, Vec<OnChainEvent>>,
    /// log of events, groupped by wallets
    pub wallets: BTreeMap<H160, Vec<OnChainEvent>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            last_block: 0,
            votings: BTreeMap::new(),
            wallets: BTreeMap::new(),
        }
    }
}
