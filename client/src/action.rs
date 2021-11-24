use crate::nice;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use web3::types::{H160, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionSignature {
    Transfer,
    // transfer which is invalid
    InvalidTransfer,
    UnknownSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingAction {
    pub action: ActionSignature,       // i.e. "Transfer"
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
                    "{:?} {} {} to 0x{}",
                    self.action,
                    nice::ceil(self.amount, 6),
                    self.token,
                    hex::encode(to)
                )
            }
            None => write!(f, "{:?}", self.action),
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

        let signature = script_data
            .iter()
            .skip(160)
            .take(4)
            .map(|x| x.clone())
            .collect::<Vec<u8>>();
        let signature_str = hex::encode(&signature);
        println!("signature={}", signature_str);
        let action = if signature_str == "a9059cbb" {
            ActionSignature::Transfer
        } else if signature_str == "9d61d234" { // invalid case of the misleading docs
            ActionSignature::InvalidTransfer
        } else {
            ActionSignature::UnknownSignature
        };
        
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
                    action,
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;
   
    #[test]
    pub fn it_reads_action_of_transfer() {
        let input: Vec<u8> = vec![0, 0, 0, 1, 85, 110, 203, 176, 49, 29, 53, 4, 145, 186, 14, 199, 224, 25, 195, 84, 215, 114, 60, 224, 0, 0, 0, 228, 182, 29, 39, 246, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 184, 105, 145, 198, 33, 139, 54, 193, 209, 157, 74, 46, 158, 176, 206, 54, 6, 235, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 169, 5, 156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 226, 39, 155, 144, 127, 2, 124, 200, 159, 231, 68, 178, 181, 207, 70, 249, 120, 229, 2, 211, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 248, 227, 108, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let va = VotingAction::from_script(&input).unwrap();
        assert!(matches!(va.action, ActionSignature::Transfer));
    }
}
