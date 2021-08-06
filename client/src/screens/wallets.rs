use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, Wallet};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::U256;

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// server side state
    pub state: AppState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Screen {
    pub fn new(state: AppState) -> Self {
        Self {
            state: state.clone(),
        }
    }

    pub fn total_with_power(&self, total_votes: U256) -> u32 {
        self.state
            .wallets
            .values()
            .map(|w| {
                if nice::pct_val(w.voting_power, total_votes, 16) >= 0.001 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn render_info(&self) -> Node<Msg> {
        let total_shares = self.state.get_shares_total();
        let total_votes = self.state.get_votes_total();
        let total_minted = self.state.get_minted_total();
        let total_vesting_members = self.state.get_vested_num();
        let total_vesting_shares = self.state.get_vested_shares();
        let total_delegating_members = self.state.get_delegating_num();
        let total_delegating_shares = self.state.get_delegating_shares();
        let total_left = self.state.get_withdrawn_num();
        node! {
            <div>
                <p style="text-align: center">
                    <span class="darken">"API3 DAO currently has "</span>
                    <strong>
                        { text(nice::int(self.state.wallets.len())) }
                    </strong>
                    <span class="darken">" participants, staking "</span>
                    <strong title={nice::amount(total_shares, 18)}>
                        { text(nice::ceil(total_shares, 18)) }
                    </strong>
                    <span class="darken">" shares. "</span>
                </p>
                <p style="text-align: center">
                    <span class="darken">" There were "</span>
                    <strong title={nice::amount(total_minted, 18)}>
                        { text(nice::ceil(total_minted, 18)) }
                    </strong>
                    <span class="darken">" API3 tokens minted and locked as staking rewards "</span>
                    {
                        if total_votes == total_shares {
                            span(vec![], vec![])
                        } else {
                            div(vec![class("warn")], vec![
                                text("There is a calculation mismatch. Numbers might be not accurate")
                            ])
                        }
                    }
                </p>
                <p style="text-align: center">
                    <span class="darken">" Stakes of "</span>
                    <strong>
                        { text(nice::int(total_vesting_members)) }
                    </strong>
                    <span class="darken">" DAO members are known to be vested, owning  "</span>
                    <strong title={nice::amount(total_vesting_shares, 18)}>
                        { text(nice::ceil(total_vesting_shares, 18)) }
                    </strong>
                    <span class="darken">" shares ("</span>
                    <strong>
                        {text(nice::pct_of(total_vesting_shares, total_shares, 18))}
                        "%"
                    </strong>
                    <span class="darken">" of current voting power)"</span>
                </p>
                <p style="text-align: center">
                    <strong>
                        { text(nice::int(total_delegating_members)) }
                    </strong>
                    <span class="darken">" DAO members are delegating their voting power of "</span>
                    <strong title={nice::amount(total_delegating_shares, 18)}>
                        { text(nice::ceil(total_delegating_shares, 18)) }
                    </strong>
                    <span class="darken">" shares ("</span>
                    <strong>
                        {text(nice::pct_of(total_delegating_shares, total_shares, 18))}
                        "%"
                    </strong>
                    <span class="darken">") to others."</span>
                </p>
                <p style="text-align: center">
                    <strong>
                        { text(nice::int(total_left)) }
                    </strong>
                    <span class="darken">" DAO members withdrew most of their stakes and left. "</span>
                </p>

            </div>
        }
    }

    pub fn render_wallet_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="c">"#"</th>
                <th class="c">"Joined"</th>
                <th class="c">"Update"</th>
                <th class="l">"Wallet"</th>
                <th class="r">"Voting Power"</th>
                <th class="r">"%"</th>
                <th class="r">"Owns"</th>
                <th class="r">"Rewards"</th>
            </tr>
        }
    }

    pub fn render_wallet_tr(&self, index: usize, w: &Wallet, total_votes: U256) -> Node<Msg> {
        let pct = nice::pct3_of(w.voting_power, total_votes, 18);
        let voting_class = if nice::pct_val(w.voting_power, total_votes, 16) >= 0.001 {
            "r enough_power"
        } else {
            "r darken"
        };
        // let rewards = self.state.get_rewards(&w.address, 10000000000u64);
        let labels = self.state.get_labels(w);
        node! {
            <tr>
                <td class="c">{text(format!("{}.", index + 1))}</td>
                <td class="c darken dt">{text(nice::date(w.created_at))}</td>
                <td class="c darken dt">{text(nice::date(w.updated_at))}</td>
                <td class="l eth-address">
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
                </td>
                <td class={voting_class} title={nice::amount(w.voting_power, 18)}>{text(nice::ceil(w.voting_power,18))}</td>
                {if pct != "000.0" {
                    node! {
                        <td class={voting_class}>{text(pct)}"%"</td>
                    }
                } else {
                    td(vec![class("r")],vec![text("-")])
                }}
                <td class="r darken" title={nice::amount(w.shares, 18)}>{text(nice::ceil(w.shares,18))}</td>
                {if w.rewards > U256::from(0) {
                    node! {
                        <td class="r" title={nice::amount(w.rewards, 18)}>{text(nice::ceil(w.rewards,18))}</td>
                    }
                } else {
                    td(vec![class("r darken")],vec![text("-")])
                }}
            </tr>
        }
    }

    pub fn render_wallet(&self, w: &Wallet, total_votes: U256) -> Node<Msg> {
        node! {
            <li>
                <div class="wallet">
                    <div>
                    <a class="eth-address" href={format!("wallets/{:?}", w.address) }>
                        { text(format!("{:?}", w.address)) }
                        {
                            if w.vested {
                                span(vec![class("vested")],vec![text(" VESTED ")])
                            } else {
                                text("")
                            }
                        }
                        {
                          if let Some(ens) = &w.ens {
                            text(format!(" {:?}", ens))
                          } else {
                            text("")
                          }
                        }
                    </a>
                    </div>
                    " "
                    <span class="amt">
                        { text(nice::amount(w.voting_power, 18)) }
                    </span>
                    " "
                    <span class="pct">
                        "("
                        {text(nice::pct_of(w.voting_power, total_votes, 18))}
                        "%"
                        {
                            span(
                                vec![],
                                if w.voting_power != w.shares {
                                    vec![
                                        text(", owning "),
                                        text(nice::pct_of(w.shares, total_votes, 18)),
                                        text("%")
                                    ]
                                } else {
                                    vec![text("")]
                                },

                            )
                        }
                        ")"
                    </span>
                </div>
            </li>
        }
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        let mut sorted: Vec<Wallet> = self.state.wallets.values().cloned().collect();
        // sorted.sort_by_key(|w| std::cmp::Reverse(w.voting_power));
        sorted.sort_by(|a, b| {
            a.voting_power
                .cmp(&b.voting_power)
                .reverse()
                .then((a.rewards).cmp(&b.rewards).reverse())
        });

        let total_votes = self.state.get_votes_total();
        node! {
            <div class="screen-wallets">
                { header::render("/wallets") }
                <div class="inner">
                    <h1>{text(format!("API3 DAO: {} Member Wallets", self.state.wallets.len()))}</h1>
                    {self.render_info()}
                    {if self.state.wallets.len() > 0 {
                        div(vec![], vec![
                            div(vec![class("desktop-only")], vec![
                                table(vec
                                    ![class("table wallets-table")],
                                    vec![
                                        thead(vec![], vec![ self.render_wallet_header() ]),
                                        tbody(vec![], sorted.iter().enumerate().map(|(i, w)| self.render_wallet_tr(i, w, total_votes)).collect::<Vec<Node<Msg>>>()),
                                    ]
                                )
                            ]),
                            div(vec![class("mobile-only")], vec![
                                ol(vec
                                    ![class("wallets-list")],
                                    sorted.iter().enumerate().map(|(_, w)| self.render_wallet(w, total_votes)).collect::<Vec<Node<Msg>>>()
                                )
                            ])
                        ])
                    } else {
                        div(vec![class("wallets-empty")], vec![
                            text("There are no members yet")
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
        let title = format!(
            "API3 DAO Explore all {} members wallets",
            self.state.wallets.len()
        );
        let description = format!("Explore API3 DAO: voting power, shares and full staking rewards history of {} members. No wallet connection is needed", self.state.wallets.len());
        PageMetaInfo::new(&title, &description)
    }
}
