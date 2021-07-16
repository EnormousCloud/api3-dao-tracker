use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::state::{AppState, Wallet};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// server side state
    pub state: AppState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        let mut sorted: Vec<Wallet> = self.state.wallets.values().cloned().collect();
        sorted.sort_by_key(|w| std::cmp::Reverse(w.voting_power));
        let total_shares = self.state.get_shares_total();
        let total_votes = self.state.get_votes_total();
        node! {
            <div class="screen-wallets">
                { header::render("/wallets") }
                <div class="inner">
                    <h1>{text(format!("API3 DAO: {} Wallets", self.state.wallets.len()))}</h1>
                    <h3>
                        "Total Shares"
                        { text(nice::amount(total_shares, 18)) }
                        { if total_votes == total_shares {
                            span(vec![], vec![])
                          } else {
                            span(vec![], vec![
                                text(format!("  Power: {}", nice::amount(total_votes, 18)))
                            ])
                          }
                        }
                    </h3>
                    {ol(vec
                        ![class("wallets-list")],
                        sorted.iter().map(|w: &Wallet| {
                            node!{
                                <li>
                                    <div class="wallet">
                                        <a class="addr" href={format!("wallets/{:?}", w.address) }>
                                            { text(format!("{:?}", w.address)) }
                                        </a>
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
                        }).collect::<Vec<Node<Msg>>>()
                    )}
                </div>
                { footer::render() }
            </div>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}