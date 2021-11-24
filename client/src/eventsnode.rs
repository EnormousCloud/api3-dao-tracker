use crate::action::ActionSignature;
use crate::events::{Api3, VotingAgent};
use crate::nice;
use crate::state::{AppState, Voting, VotingDetails};
use sauron::prelude::*;
use std::collections::BTreeMap;
use web3::types::{H160, U256};

pub fn hl_text<T>(name: &str) -> Node<T> {
    node!(<strong style="color: var(--color-panel-title)">{text(name)}" "</strong>)
}

pub fn normal_text<T>(name: &str) -> Node<T> {
    node!(<span> " " {text(name)}" "</span>)
}

pub fn wrap_line<T>(nodes: Vec<Option<Node<T>>>) -> Node<T> {
    span(vec![], nodes.into_iter().flatten().collect())
}

pub fn wrap_address<T>(addr: H160) -> Node<T> {
    node!(<a href={format!("wallets/{:?}", addr)}>{text(format!("{:?}", addr))}</a>)
}

pub fn wrap_vote<T>(
    vote_id: U256,
    agent: &VotingAgent,
    votings: &BTreeMap<u64, Voting>,
) -> Option<Node<T>> {
    let vi = crate::events::voting_to_u64(agent, vote_id.as_u64());
    let badge = match agent {
        VotingAgent::Primary => span(vec![class("badge badge-primary")], vec![text("Primary")]),
        VotingAgent::Secondary => span(
            vec![class("badge badge-secondary")],
            vec![text("Secondary")],
        ),
    };
    match votings.get(&vi) {
        Some(v) => Some(node!(
            <span>
                " "
                {badge}
                <a href={format!("votings/{}", v.key()) }>
                    <strong>{text(v.title.clone())}</strong>
                </a>
                " "
            </span>)),
        None => None,
    }
}

pub fn wrap_vote_details<T>(details: &Option<VotingDetails>) -> Node<T> {
    if let Some(d) = details {
        if let Some(action) = &d.action {
            if let Some(w) = action.wallet {
                return node!(
                    <small class="vote-script">
                        {match &action.action {
                            ActionSignature::Transfer => text(format!("{:?}", action.action)),
                            _ => node!{ 
                                <span class="badge badge-withdrawn">
                                    {text(format!("{:?}", action.action))}
                                </span>
                            }
                        }}
                        " "
                        {wrap_amt_dec(action.amount, action.decimals)}
                        " "
                        {text(action.token.clone())}
                        " to "
                        {wrap_address(w)}
                    </small>
                );
            } else {
                return node!(<small>{text(action)}</small>);
            }
        }
    }
    div(vec![], vec![])
}

pub fn wrap_amt_dec<T>(val: U256, decimals: usize) -> Node<T> {
    node!(<strong style="color: var(--color-panel-title)" title={nice::amount(val, decimals)}>{text(nice::ceil(val,decimals))}</strong>)
}

pub fn wrap_amt<T>(val: U256) -> Node<T> {
    node!(<strong style="color: var(--color-panel-title)" title={nice::amount(val, 18)}>{text(nice::ceil(val,18))}</strong>)
}

pub fn wrap_time<T>(val: U256) -> Node<T> {
    let v = val.as_u64();
    node!(<strong style="color: var(--color-panel-title)" title={format!("{}", v)}>{text(nice::date(v))}</strong>)
}

pub fn wrap_label<T>(label: &str, node: Node<T>) -> Node<T> {
    span(vec![], vec![text(label), node])
}

pub fn entry_node<T>(entry: &Api3, addr: H160, state: &AppState) -> Node<T> {
    match entry {
        Api3::Delegated {
            from,
            to,
            shares,
            total_delegated_to,
        } => wrap_line(vec![
            Some(hl_text("Delegated")),
            Some(normal_text(" ")),
            if *from != addr {
                Some(wrap_label("from: ", wrap_address(*from)))
            } else {
                None
            },
            if *to != addr {
                Some(wrap_label("to: ", wrap_address(*to)))
            } else {
                None
            },
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
            Some(normal_text("total_delegated_to: ")),
            Some(wrap_amt(*total_delegated_to)),
        ]),
        Api3::DelegatedV0 { from, to, shares } => wrap_line(vec![
            Some(hl_text("Delegated")),
            Some(normal_text(" ")),
            if *from != addr {
                Some(wrap_label("from: ", wrap_address(*from)))
            } else {
                None
            },
            if *to != addr {
                Some(wrap_label("to: ", wrap_address(*to)))
            } else {
                None
            },
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
        ]),
        Api3::Undelegated {
            from,
            to,
            shares,
            total_delegated_to,
        } => wrap_line(vec![
            Some(hl_text("Undelegated")),
            Some(normal_text(" ")),
            if *from != addr {
                Some(wrap_label("from: ", wrap_address(*from)))
            } else {
                None
            },
            if *to != addr {
                Some(wrap_label("to: ", wrap_address(*to)))
            } else {
                None
            },
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
            Some(normal_text("total_delegated_to: ")),
            Some(wrap_amt(*total_delegated_to)),
        ]),
        Api3::UndelegatedV0 { from, to, shares } => wrap_line(vec![
            Some(hl_text("Undelegated")),
            if *from != addr {
                Some(wrap_label("from: ", wrap_address(*from)))
            } else {
                None
            },
            if *to != addr {
                Some(wrap_label("to: ", wrap_address(*to)))
            } else {
                None
            },
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
        ]),
        Api3::UpdatedDelegation {
            user,
            delegate,
            delta: _,
            shares,
            total_delegated_to,
        } => wrap_line(vec![
            Some(hl_text("UpdatedDelegation")),
            if *user != addr {
                Some(wrap_label("user: ", wrap_address(*user)))
            } else {
                None
            },
            if *delegate != addr {
                Some(wrap_label("delegate: ", wrap_address(*delegate)))
            } else {
                None
            },
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
            Some(normal_text("total_delegated_to: ")),
            Some(wrap_amt(*total_delegated_to)),
        ]),
        Api3::Staked {
            user: _,
            amount,
            minted_shares,
            user_unstaked,
            user_shares,
            total_shares,
            total_stake,
        } => wrap_line(vec![
            Some(hl_text("Staked")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("minted_shares: ")),
            Some(wrap_amt(*minted_shares)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
            Some(normal_text("user_shares: ")),
            Some(wrap_amt(*user_shares)),
            Some(normal_text("total_shares: ")),
            Some(wrap_amt(*total_shares)),
            Some(normal_text("total_stake: ")),
            Some(wrap_amt(*total_stake)),
        ]),
        Api3::StakedV0 {
            user: _,
            amount,
            minted_shares,
        } => wrap_line(vec![
            Some(hl_text("Staked")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("minted_shares: ")),
            Some(wrap_amt(*minted_shares)),
        ]),
        Api3::Unstaked {
            user: _,
            amount,
            user_unstaked,
            total_shares,
            total_stake,
        } => wrap_line(vec![
            Some(hl_text("Unstaked")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
            Some(normal_text("total_shares: ")),
            Some(wrap_amt(*total_shares)),
            Some(normal_text("total_stake: ")),
            Some(wrap_amt(*total_stake)),
        ]),
        Api3::UnstakedV0 { user: _, amount } => wrap_line(vec![
            Some(hl_text("Unstaked")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
        ]),

        Api3::ScheduledUnstake {
            user: _,
            amount,
            shares,
            scheduled_for,
            user_shares,
        } => wrap_line(vec![
            Some(hl_text("ScheduledUnstake")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
            Some(normal_text("scheduled_for: ")),
            Some(wrap_time(*scheduled_for)),
            Some(normal_text("user_shares: ")),
            Some(wrap_amt(*user_shares)),
        ]),

        Api3::ScheduledUnstakeV0 {
            user: _,
            amount,
            shares,
            scheduled_for,
        } => wrap_line(vec![
            Some(hl_text("ScheduledUnstake")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("shares: ")),
            Some(wrap_amt(*shares)),
            Some(normal_text("scheduled_for: ")),
            Some(wrap_time(*scheduled_for)),
        ]),
        Api3::Deposited {
            user: _,
            amount,
            user_unstaked,
        } => wrap_line(vec![
            Some(hl_text("Deposited")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
        ]),
        Api3::DepositedV0 { user: _, amount } => wrap_line(vec![
            Some(hl_text("Deposited")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
        ]),
        Api3::DepositedVesting {
            user: _,
            amount,
            start,
            end,
            user_unstaked,
            user_vesting,
        } => wrap_line(vec![
            Some(hl_text("DepositedVesting")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("start: ")),
            Some(wrap_time(*start)),
            Some(normal_text("end: ")),
            Some(wrap_time(*end)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
            Some(normal_text("user_vesting: ")),
            Some(wrap_amt(*user_vesting)),
        ]),
        Api3::DepositedByTimelockManager {
            user: _,
            amount,
            user_unstaked,
        } => wrap_line(vec![
            Some(hl_text("DepositedByTimelockManager")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
        ]),

        Api3::VestedTimelock {
            user: _,
            amount,
            user_vesting,
        } => wrap_line(vec![
            Some(hl_text("VestedTimelock")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("user_vesting: ")),
            Some(wrap_amt(*user_vesting)),
        ]),

        Api3::Withdrawn {
            user: _,
            amount,
            user_unstaked,
        } => wrap_line(vec![
            Some(hl_text("Withdrawn")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("user_unstaked: ")),
            Some(wrap_amt(*user_unstaked)),
        ]),
        Api3::WithdrawnV0 { user: _, amount } => wrap_line(vec![
            Some(hl_text("Withdrawn")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
        ]),

        Api3::WithdrawnToPool {
            recipient: _,
            api3_pool_address: _,
            beneficiary: _,
        } => wrap_line(vec![Some(hl_text("WithdrawnToPool"))]),

        // never happened yet
        Api3::PaidOutClaim {
            recipient: _,
            amount,
            total_stake,
        } => wrap_line(vec![
            Some(hl_text("PaidOutClaim")),
            Some(normal_text("amount: ")),
            Some(wrap_amt(*amount)),
            Some(normal_text("total_stake: ")),
            Some(wrap_amt(*total_stake)),
        ]),

        // Voting
        Api3::StartVote {
            agent,
            vote_id,
            creator: _,
            metadata: _,
        } => wrap_line(vec![
            Some(hl_text("StartVote")),
            wrap_vote(*vote_id, agent, &state.votings),
        ]),
        Api3::CastVote {
            agent,
            vote_id,
            voter: _,
            supports,
            stake,
        } => wrap_line(vec![
            Some(hl_text("CastVote")),
            wrap_vote(*vote_id, agent, &state.votings),
            Some(hl_text(if *supports { "YEA" } else { "NEI" })),
            Some(normal_text("stake: ")),
            Some(wrap_amt(*stake)),
        ]),
        Api3::SetVestingAddresses { addresses: _ } => {
            wrap_line(vec![Some(hl_text("SetVestingAddresses"))])
        }
        _ => text(format!("{:?}", entry)),
    }
}
