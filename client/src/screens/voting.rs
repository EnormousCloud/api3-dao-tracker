use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::events::{self, Api3, VotingAgent};
use crate::nice;
use crate::router::link_eventlog;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, LabelBadge, OnChainEvent, Wallet};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::{H160, U256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    // ID of the voting
    pub vote_ref: u64,
    // ID of the voting
    pub vote_id: u64,
    // agent of the voting
    pub agent: VotingAgent,
    /// server side state
    pub state: AppState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Screen {
    pub fn new(state: AppState, vote_ref: u64) -> Self {
        let (agent, vote_id) = events::voting_from_u64(vote_ref);
        Self {
            vote_ref,
            vote_id,
            agent,
            state: state.clone(),
        }
    }

    pub fn get_labels(&self, w: &Wallet) -> Vec<LabelBadge> {
        let mut labels: Vec<LabelBadge> = vec![];
        if w.vested || self.state.is_vested_deposit(&w.address) {
            labels.push(LabelBadge::new(
                "badge-vested",
                "vested",
                "Some shares of this member are vested",
            ));
        }
        if w.supporter {
            labels.push(LabelBadge::new(
                "badge-supporter",
                "supporter",
                "API3 tokens are not vested, can withdraw, but never did",
            ));
        }
        if w.withdrawn > U256::from(0) {
            labels.push(LabelBadge::new(
                "badge-withdrawn",
                "withdrawn",
                "Withdrew tokens in the past",
            ));
        } else if let Some(_) = w.scheduled_unstake {
            if w.withdrawn == U256::from(0) {
                labels.push(LabelBadge::new(
                    "badge-unstaking",
                    "unstaking",
                    "In the process of withdrawing",
                ));
            }
        } else if !w.supporter && w.deposited > U256::from(0) && w.voting_power == U256::from(0) {
            labels.push(LabelBadge::new(
                "badge-not-staking",
                "deposited, not staking",
                "Deposited tokens but not staking them",
            ));
        }
        labels
    }

    pub fn link_wallet(&self, addr: H160) -> Node<Msg> {
        match self.state.wallets.get(&addr) {
            Some(w) => {
                let labels = self.get_labels(w);
                node! {
                    <a href={format!("wallets/{:?}", w.address)}>
                        <div>
                            {span(vec![class("badges")], labels.iter().map(|v| {
                                let title = format!("{}", v.title);
                                node! {
                                    <span class={format!("badge {}", v.class)} title={title}>{text(v.text.clone().as_str())}</span>
                                }
                            }).collect::<Vec<Node<Msg>>>())}
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

    pub fn render_event_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="c">"#"</th>
                <th class="c">"Date"</th>
                <th class="c">"Block Number"</th>
                <th class="c">"Event"</th>
                <th class="l">"User"</th>
                <th class="l">"Cast"</th>
                <th class="l">"Shares"</th>
                <th class="l">"%"</th>
            </tr>
        }
    }
    pub fn render_event(&self, _e: &OnChainEvent, _total_shares: U256) -> Node<Msg> {
        div(vec![], vec![])
    }

    pub fn render_event_tr(&self, index: usize, e: &OnChainEvent, total_shares: U256) -> Node<Msg> {
        let event: String = (match e.entry {
            Api3::StartVote {
                agent: _,
                vote_id: _,
                creator: _,
                metadata: _,
            } => "StartVote",
            Api3::CastVote {
                agent: _,
                vote_id: _,
                voter: _,
                supports: _,
                stake: _,
            } => "CastVote",
            Api3::ExecuteVote {
                agent: _,
                vote_id: _,
            } => "ExecuteVote",
            _ => "",
        })
        .to_owned();

        let shares: Option<U256> = match e.entry {
            Api3::CastVote {
                agent: _,
                vote_id: _,
                voter: _,
                supports: _,
                stake,
            } => Some(stake.clone()),
            _ => None,
        };

        let voter: Option<H160> = match e.entry {
            Api3::StartVote {
                agent: _,
                vote_id: _,
                creator,
                metadata: _,
            } => Some(creator.clone()),
            Api3::CastVote {
                agent: _,
                vote_id: _,
                voter,
                supports: _,
                stake: _,
            } => Some(voter.clone()),
            _ => None,
        };
        let supports: Option<bool> = match e.entry {
            Api3::CastVote {
                agent: _,
                vote_id: _,
                voter: _,
                supports,
                stake: _,
            } => Some(supports),
            _ => None,
        };

        node! {
            <tr>
                <td class="c">{text(format!("{}.", index + 1))}</td>
                <td class="c darken dt">{text(nice::date(e.tm))}</td>
                <td class="c">{link_eventlog(self.state.chain_id, e.block_number, e.tx)}</td>
                <td class="c darken">{text(event)}</td>
                <td class="l eth-address">{
                    match voter {
                        Some(x) => self.link_wallet(x),
                        None => text(""),
                    }
                }
                </td>
                <td class="c">{
                    match supports {
                        Some(true) => text("Supports"),
                        Some(false) => text("Rejects"),
                        _ => text(""),
                    }
                }
                </td>
                {
                    match shares {
                        Some(shares) => node!{
                            <td class="r" title={nice::amount(shares, 18)}>{text(nice::ceil(shares,18))}</td>
                        },
                        _ => node!{ <td class="r"></td> }
                    }
                }
                {
                    match shares {
                        Some(shares) => {
                            let pct = nice::pct3_of(shares, total_shares, 18);
                            if pct != "000.0" {
                                node!{ <td class={"r darken shares-pct"}>{text(pct)}"%"</td> }
                            } else {
                                node!{ <td class="r"></td> }
                            }
                        },
                        _ => node!{ <td class="r"></td> }
                    }
                }
            </tr>
        }
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        let v = match self.state.votings.get(&self.vote_ref) {
            Some(v) => v,
            None => return err_box("voting was not found"),
        };

        let subtitle = format!(
            "API3 DAO {} Proposal Voting",
            if v.primary { "Primary" } else { "Secondary" }
        );
        let total = v.votes_total;
        let pct_required = if v.primary { 50u64 } else { 15u64 };
        let required = total * U256::from(pct_required) / U256::from(100);
        let pct_yes = nice::pct3_of(v.voted_yes, v.votes_total, 18);
        let pct_no = nice::pct3_of(v.voted_no, v.votes_total, 18);
        let sorted: Vec<OnChainEvent> = self.state.votings_events.get(&v.as_u64()).unwrap().clone();
        let decision = "text-align:center; border: 1px #888 solid; padding: 30px";
        node! {
            <div class="screen-voting">
                { header::render("/votings") }
                <div class="inner">
                    <h1>{text(v.title.clone())}</h1>
                    <h2 style="text-align: center">{text(subtitle)}</h2>
                    <p style="text-align: center; line-height: 1.5">{text(v.description.clone())}</p>
                    <p style="text-align: center; line-height: 3">
                        <strong>{text(v.trigger_str())}</strong>
                    </p>
                    <p style="text-align: center">
                        <span class="darken">"At the time of the voting DAO had "</span>
                        <strong title={nice::amount(total, 18)}>{ text(nice::ceil(total, 18)) }</strong>
                        <span class="darken">" shares staked, "</span>
                        <strong title={nice::amount(required, 18)}>{ text(nice::ceil(required, 18)) }</strong>
                        <span class="darken">" shares are required for this proposal to be accepted"</span>
                    </p>
                    {if v.voted_yes > U256::from(0) {
                        let cls = if v.voted_yes > required { "accent" } else { "" };
                        node! {
                            <p class={cls} style="text-align: center">
                                <strong title={nice::amount(v.voted_yes, 18)}>{ text(nice::ceil(v.voted_yes, 18)) }</strong>
                                <span class="darken">" votes supported this proposal ("</span>
                                {text(pct_yes.clone())}
                                <span class="darken">"%) "</span>
                            </p>
                        }
                    } else {
                        text("")
                    }}
                    {if v.voted_no > U256::from(0) {
                        let cls = if v.voted_no > required { "warning" } else { "" };
                        node! {
                            <p class={cls} style="text-align: center">
                                <strong title={nice::amount(v.voted_no, 18)}>{ text(nice::ceil(v.voted_no, 18)) }</strong>
                                <span class="darken">" votes against this proposal ("</span>
                                {text(pct_no.clone())}
                                <span class="darken">"%) "</span>
                            </p>
                        }
                    } else {
                        node! {
                            <p style="text-align: center" class="darken">
                                "Nobody voted against"
                            </p>
                        }
                    }}

                    {if v.executed {
                        node! { <h3 style={decision} class="accent">"ACCEPTED AND EXECUTED"</h3> }
                    } else if v.voted_yes > required {
                        node! { <h3 style={decision} class="accent">"PROPOSAL IS PASSING, NOT EXECUTED"</h3> }
                    } else if v.voted_no > required {
                        node! { <h3 style={decision} class="warning">"PROPOSAL REJECTED"</h3> }
                    } else {
                        text("")
                    }}

                    <h2 style="text-align: center">"Voting History Log"</h2>
                    {if self.state.votings_events.len() > 0 {
                        div(vec![], vec![
                            div(vec![class("desktop-only")], vec![
                                table(vec
                                    ![class("table events-table")],
                                    vec![
                                        thead(vec![], vec![ self.render_event_header() ]),
                                        tbody(vec![], sorted.iter().enumerate().map(|(i, e)| self.render_event_tr(i, e, total)).collect::<Vec<Node<Msg>>>()),
                                    ]
                                )
                            ]),
                            div(vec![class("mobile-only")], vec![
                                ol(vec
                                    ![class("events-list")],
                                    sorted.iter().enumerate().map(|(_, e)| self.render_event(e, total)).collect::<Vec<Node<Msg>>>()
                                )
                            ])
                        ])
                    } else {
                        div(vec![class("events-empty")], vec![
                            text("There were no votings events in the DAO")
                        ])
                    }}
                </div>
                { footer::render(&self.state) }
            </div>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}

impl MetaProvider for Screen {
    fn meta(&self) -> PageMetaInfo {
        let description =
            "Explore API3 DAO proposal full voting history. No wallet connection is needed"
                .to_owned();
        let title = match self.state.votings.get(&self.vote_ref) {
            Some(v) => format!(
                "API3 DAO {} Proposal Voting History",
                if v.primary { "Primary" } else { "Secondary" }
            ),
            None => "API3 DAO Voting was not found".to_owned(),
        };
        PageMetaInfo::new(&title, &description)
    }
}
