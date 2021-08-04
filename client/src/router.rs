use crate::events::Api3;
use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;
use web3::types::{H160, H256};

pub fn link_eventlog<T>(chain_id: u64, block_number: u64, tx: H256) -> Node<T> {
    let link = match chain_id {
        1 => Some(format!("https://etherscan.io/tx/{:?}#eventlog", tx)),
        4 => Some(format!("https://rinkeby.etherscan.io/tx/{:?}#eventlog", tx)),
        _ => None,
    };
    match link {
        Some(link) => node! {
            <a href={link} rel="nofollow noopener noreferrer" target="_blank">
                {text(nice::int(block_number))}
            </a>
        },
        None => text(nice::int(block_number)),
    }
}

pub fn link<T>(link: String) -> Node<T> {
    if link.len() == 0 {
        return text("");
    }
    node! {
        <a href={link.clone()} rel="nofollow noopener noreferrer" target="_blank">
            {text(link.clone())}
        </a>
    }
}

pub fn link_wallet<T>(state: &AppState, addr: H160) -> Node<T> {
    match state.wallets.get(&addr) {
        Some(w) => {
            let labels = state.get_labels(w);
            node! {
                <a href={format!("wallets/{:?}", w.address)}>
                    <div>
                        {span(vec![class("badges")], labels.iter().map(|v| {
                            let title = format!("{}", v.title);
                            node! {
                                <span class={format!("badge {}", v.class)} title={title}>{text(v.text.clone().as_str())}</span>
                            }
                        }).collect::<Vec<Node<T>>>())}
                        {match &w.ens {
                            Some(ens) => strong(vec![class("ens")],vec![text(ens)]),
                            None => span(vec![],vec![]),
                        }}
                    </div>
                    <div>{text(format!("{:?}", w.address))}</div>
                </a>
            }
        }
        None => span(vec![], vec![text(format!("{:?}", addr))]),
    }
}

pub fn text_entry<T>(entry: &Api3) -> Node<T> {
    match entry {
        // Delegated {
        //     from: H160,
        //     to: H160,
        //     shares: U256,
        //     total_delegated_to: U256,
        // },
        // DelegatedV0 {
        //     from: H160,
        //     to: H160,
        //     shares: U256,
        // },
        // Undelegated {
        //     from: H160,
        //     to: H160,
        //     shares: U256,
        //     total_delegated_to: U256,
        // },
        // UndelegatedV0 {
        //     from: H160,
        //     to: H160,
        //     shares: U256,
        // },
        // UpdatedDelegation {
        //     user: H160,
        //     delegate: H160,
        //     delta: bool,
        //     shares: U256,
        //     total_delegated_to: U256,
        // },
        // Staked {
        //     user: H160,
        //     amount: U256,
        //     minted_shares: U256,
        //     user_unstaked: U256,
        //     user_shares: U256,
        //     total_shares: U256,
        //     total_stake: U256,
        // },
        // StakedV0 {
        //     user: H160,
        //     amount: U256,
        //     minted_shares: U256,
        // },
        // Unstaked {
        //     user: H160,
        //     amount: U256,
        //     user_unstaked: U256,
        //     total_shares: U256,
        //     total_stake: U256,
        // },
        // UnstakedV0 {
        //     user: H160,
        //     amount: U256,
        // },
        // ScheduledUnstake {
        //     user: H160,
        //     amount: U256,
        //     shares: U256,
        //     scheduled_for: U256,
        //     user_shares: U256,
        // },
        // ScheduledUnstakeV0 {
        //     user: H160,
        //     amount: U256,
        //     shares: U256,
        //     scheduled_for: U256,
        // },
        // Deposited {
        //     user: H160,
        //     amount: U256,
        //     user_unstaked: U256,
        // },
        // DepositedV0 {
        //     user: H160,
        //     amount: U256,
        // },
        // DepositedVesting {
        //     user: H160,
        //     amount: U256,
        //     start: U256,
        //     end: U256,
        //     user_unstaked: U256,
        //     user_vesting: U256,
        // },
        // DepositedByTimelockManager {
        //     user: H160,
        //     amount: U256,
        //     user_unstaked: U256,
        // },
        // VestedTimelock {
        //     user: H160,
        //     amount: U256,
        //     user_vesting: U256,
        // },
        // Withdrawn {
        //     user: H160,
        //     amount: U256,
        //     user_unstaked: U256,
        // },
        // WithdrawnV0 {
        //     user: H160,
        //     amount: U256,
        // },
        // WithdrawnToPool {
        //     recipient: H160,
        //     api3_pool_address: H160,
        //     beneficiary: H160,
        // },
        // UpdatedLastProposalTimestamp {
        //     user: H160,
        //     last_proposal_timestamp: U256,
        //     voting_app: H160,
        // },
        // SetStakeTarget {
        //     stake_target: U256,
        // },
        // MintedReward {
        //     epoch_index: U256,
        //     amount: U256,
        //     new_apr: U256,
        //     total_stake: U256,
        // },
        // MintedRewardV0 {
        //     epoch_index: U256,
        //     amount: U256,
        //     new_apr: U256,
        // },
        // // never happened yet
        // PaidOutClaim {
        //     recipient: H160,
        //     amount: U256,
        //     total_stake: U256,
        // },
    
        // // Voting
        // StartVote {
        //     agent: VotingAgent,
        //     vote_id: U256,
        //     creator: H160,
        //     metadata: String,
        // } => 
        // CastVote {
        //     agent: VotingAgent,
        //     vote_id: U256,
        //     voter: H160,
        //     supports: bool,
        //     stake: U256,
        // },
        // ExecuteVote {
        //     agent: VotingAgent,
        //     vote_id: U256,
        // } 
    
        Api3::SetVestingAddresses { addresses:  _ } => text("SetVestingAddresses"),
        _ => text(format!("{:?}", entry)),
    }
}