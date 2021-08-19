use crate::nice;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use web3::types::{H160, U256};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VotingAction {
    pub action: String,       // i.e. "Transfer"
    pub token: String,        // i.e. "USDC"
    pub amount: U256,         // amount to be transferred
    pub decimals: usize,      // decimals for the token, i.e. 18
    pub wallet: Option<H160>, // wallet-destination in case of Transfer or similar methods
}
impl fmt::Display for VotingAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.wallet {
            Some(to) => {
                write!(
                    f,
                    "{} {} {} to 0x{}",
                    self.action,
                    nice::ceil(self.amount, 6),
                    self.token,
                    hex::encode(to)
                )
            }
            None => write!(f, "{}", self.action),
        }
    }
}

pub struct TokenDescriptor {
    pub name: String,
    pub decimals: usize,
    pub addr: Vec<u8>,
}

impl TokenDescriptor {
    pub fn new(name: &str, decimals: usize, addr: H160) -> Self {
        Self {
            name: name.to_owned(),
            decimals,
            addr: addr.as_bytes().into(),
        }
    }
}

impl VotingAction {
    pub fn from_script(script_data: &Vec<u8>) -> Option<Self> {
        if script_data.len() == 0 {
            return None;
        }

        let mut tokens: Vec<TokenDescriptor> = vec![];
        tokens.push(TokenDescriptor::new(
            "USDC",
            6,
            hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").into(),
        ));
        tokens.push(TokenDescriptor::new(
            "API3",
            6,
            hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").into(),
        ));

        for t in tokens {
            let addr: Vec<u8> = script_data
                .iter()
                .skip(0x20 + 12)
                .take(20)
                .map(|x| x.clone())
                .collect();
            if addr == t.addr.as_slice() {
                // not very accurante, but most likely this is Transfer
                // and it can work
                let offset = 32 + 32 + 32 + 32 + 32 + 4 + 12;
                let to: Vec<u8> = script_data
                    .iter()
                    .skip(offset)
                    .take(20)
                    .map(|x| x.clone())
                    .collect();
                let amt: Vec<u8> = script_data
                    .iter()
                    .skip(offset + 20 + 16)
                    .take(16)
                    .map(|x| x.clone())
                    .collect();
                let amt_hex = format!("0x{}", hex::encode(amt));
                let amount: U256 = U256::from_str(&amt_hex).unwrap();
                return Some(Self { 
                    action: "Transfer".to_owned(),
                    amount,
                    wallet: Some(H160::from_slice(&to)),
                    token: t.name,
                    decimals: t.decimals,
                 });
            }
        }

        return None;
    }
}
