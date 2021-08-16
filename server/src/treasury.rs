use crate::contracts::Erc20Contract;
use client::state::Treasury;
use std::collections::BTreeMap;
use web3::types::{H160};

pub async fn get_treasury<T>(
    web3: &web3::Web3<T>,
    name: String,
    wallet: H160,
    contracts: &BTreeMap<String, H160>,
) -> Treasury
where
    T: web3::Transport,
{
    let mut treasury = Treasury::new(name.clone(), wallet.clone());
    for (token, addr) in contracts {
        let contract = Erc20Contract::new(&web3, *addr);
        let val = contract.get_balance(wallet).await;
        if let Some(v) = &val {
            treasury.balances.insert(token.clone(), v.clone());
        }
    }
    let dt = chrono::Utc::now().naive_utc();
    treasury.updated_at = dt.timestamp();
    treasury
}

pub async fn read_treasuries<T>(
    web3: &web3::Web3<T>,
    tokens: &BTreeMap<String, H160>,
    wallets: &BTreeMap<String, H160>,
) -> BTreeMap<String, Treasury>
where
    T: web3::Transport,
{
    let mut res = BTreeMap::new();
    for (name, wallet) in wallets {
        res.insert(name.to_owned(), get_treasury(web3, name.clone(), wallet.clone(), tokens).await);
    }
    res
}
