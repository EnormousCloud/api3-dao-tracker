use crate::events::{Api3, VotingAgent};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Voting {
    pub primary: bool,
    pub vote_id: u64,
    pub creator: H160,
    pub metadata: String,
    pub voted_yes: U256,
    pub voted_no: U256,
    pub list_yes: Vec<H160>,
    pub list_no: Vec<H160>,
    pub votes_total: U256,
    pub executed: bool,
}

impl Voting {
    pub fn as_u64(&self) -> u64 {
        let agent = if self.primary {
            VotingAgent::Primary
        } else {
            VotingAgent::Secondary
        };
        crate::events::voting_to_u64(&agent, self.vote_id)
    }
    pub fn key(&self) -> String {
        let agent = if self.primary {
            VotingAgent::Primary
        } else {
            VotingAgent::Secondary
        };
        crate::events::voting_to_string(&agent, self.vote_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wallet {
    pub address: H160,
    pub ens: Option<String>,
    pub deposited: U256,
    pub withdrawn: U256,
    pub staked: U256,
    pub shares: U256,
    pub delegated_to: Option<H160>,
    pub voting_power: U256,
    pub votes: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// the block of the last event
    pub last_block: u64,
    /// map of votings
    pub votings: BTreeMap<u64, Voting>,
    /// log of events, grouped by votings
    pub votings_events: BTreeMap<u64, Vec<OnChainEvent>>,
    /// map of wallets
    pub wallets: BTreeMap<H160, Wallet>,
    /// log of events, groupped by wallets
    pub wallets_events: BTreeMap<H160, Vec<OnChainEvent>>,
    /// log of all events, groupped by date
    pub events: Vec<OnChainEvent>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            last_block: 0,
            votings: BTreeMap::new(),
            wallets: BTreeMap::new(),
            votings_events: BTreeMap::new(),
            wallets_events: BTreeMap::new(),
            events: vec![],
        }
    }

    pub fn get_voting_power_of(&self, voter: &H160) -> U256 {
        match self.wallets.get(voter) {
            Some(wallet) => wallet.voting_power,
            None => U256::from(0),
        }
    }

    pub fn get_votes_total(&self) -> U256 {
        self.wallets.values().map(|w| w.voting_power).fold(U256::from(0), |a, b| {
            a + b
        })
    }

    pub fn update(&mut self, e: OnChainEvent, log: web3::types::Log) -> () {
        println!("update {:?}", e);

        log.block_number.map(|block_number| {
            self.last_block = block_number.as_u64();
        });
        self.events.push(e.clone());

        e.entry.get_wallets().iter().for_each(|wallet| {
            if !self.wallets_events.contains_key(&wallet) {
                self.wallets_events.insert(wallet.clone(), vec![]);
                let mut w = Wallet::default();
                w.address = wallet.clone();
                w.created_at = e.tm;
                self.wallets.insert(wallet.clone(), w);
            }
            if let Some(w) = self.wallets_events.get_mut(&wallet) {
                w.push(e.clone());
            }
        });
        e.entry.get_voting().map(|id| {
            if !self.votings_events.contains_key(&id) {
                self.votings_events.insert(id, vec![]);
            }
            if let Some(v) = self.votings_events.get_mut(&id) {
                v.push(e.clone());
            }
        });
        match &e.entry {
            Api3::StartVote {
                agent,
                vote_id,
                creator,
                metadata,
            } => {
                let primary = match agent {
                    VotingAgent::Primary => true,
                    VotingAgent::Secondary => false,
                };
                let v = Voting {
                    primary,
                    vote_id: vote_id.as_u64(),
                    creator: creator.clone(),
                    metadata: metadata.clone(),
                    votes_total: self.get_votes_total(),
                    voted_yes: self.get_voting_power_of(&creator),
                    voted_no: U256::from(0),
                    list_yes: vec![creator.clone()],
                    list_no: vec![],
                    executed: false,
                };
                self.votings.insert(v.as_u64(), v);
            },
            Api3::CastVote {
                agent,
                vote_id,
                voter,
                supports,
                stake,
            } => {
                let key = crate::events::voting_to_u64(agent, vote_id.as_u64());
                if let Some(v) = self.votings.get_mut(&key) {
                    if *supports {
                        v.voted_yes += *stake;
                        v.list_yes.push(voter.clone())
                    } else {
                        v.voted_no += *stake;
                        v.list_no.push(voter.clone())
                    }
                }
            },
            Api3::ExecuteVote {
                agent,
                vote_id,
            } => {
                let key = crate::events::voting_to_u64(agent, vote_id.as_u64());
                if let Some(v) = self.votings.get_mut(&key) {
                    v.executed = true;
                }
            }
            _ => {}
        };
    }
}
