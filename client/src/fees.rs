use std::collections::HashSet;

use crate::nice;
use crate::state::OnChainEvent;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use web3::types::TransactionReceipt as Receipt;
use web3::types::{Transaction, H256, U256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxFee {
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount (limit on transaction creation)
    pub gas: U256,
    /// Gas that was actually used from receipt
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<U256>,
    /// USD equivalent of the price paid for gas
    pub usd: Option<f64>,
}

impl TxFee {
    pub fn new(tx: &Transaction, receipt: &Receipt, dt: NaiveDateTime) -> Self {
        let eth = match receipt.gas_used {
            Some(gas_used) => gas_used * tx.gas_price,
            None => tx.gas * tx.gas_price,
        };
        let usd = crate::usdprice::coin_price_at("ethereum", eth, 18, dt);
        Self {
            gas_price: tx.gas_price,
            gas: tx.gas,
            gas_used: receipt.gas_used,
            usd,
        }
    }

    pub fn to_string(&self) -> String {
        let mut pieces: Vec<String> = vec![];
        pieces.push(match self.gas_used {
            Some(gas_used) => format!("Gas Used: {}", gas_used),
            None => format!("Gas Limit: {}", self.gas),
        });
        pieces.push(format!("Gas Price: {} GWei", nice::dec(self.gas_price, 9)));
        if let Some(usd) = self.usd {
            pieces.push(format!("Est ${}", usd));
        }
        pieces.join(", ")
    }
}

