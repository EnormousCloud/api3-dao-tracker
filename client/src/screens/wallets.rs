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
        let total = self.state.get_shares_total();
        node! {
            <div class="screen-wallets">
                { header::render("/wallets") }
                <div class="inner">
                    <h1>{text(format!("API3 DAO: {} Wallets", self.state.wallets.len()))}</h1>
                    <h3>
                        "Total Shares"
                        { text(nice::amount(total, 18)) }
                    </h3>
                    <div class="warn">"Votes delegation is not counted. Below is shares ownership distribution"</div>
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
                                            { text(nice::amount(w.shares, 18)) }
                                        </span>
                                        " "
                                        <span class="pct">
                                            "("
                                            {text(nice::pct_of(w.shares, total, 18))}
                                            "%)"
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
