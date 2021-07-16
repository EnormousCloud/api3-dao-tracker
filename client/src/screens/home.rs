use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// server side state
    pub state: AppState,
}

impl Screen {
    pub fn new(state: AppState) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

pub fn no_node<T>() -> Node<T> {
    div(vec![], vec![])
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-home">
                { header::render("") }
                <div class="inner">
                    <div class="centered">
                        <h1>"API3 DAO Tracker"</h1>
                        <h2>"on-chain analytics"</h2>
                        <div class="stats-row">
                            <a href="./wallets">
                                { text(nice::int(self.state.wallets.len())) }
                                " DAO members"
                            </a>
                        </div>
                        {if self.state.votings.len() > 0 {
                            node! {
                                <div class="stats-row">
                                    <a href="./votings">
                                        { text(nice::int(self.state.votings.len())) }
                                        " DAO votings"
                                    </a>
                                </div>
                            }
                        } else {
                            no_node()
                        }}
                        {if let Some(last_epoch) = self.state.last_epoch {
                            node! {
                                <div class="stats-row">
                                    {
                                      if let Some(last_minted) = self.state.last_minted {
                                        text(format!("{} API3 tokens minted", nice::int(nice::dec(last_minted, 18))))
                                      } else {
                                        text("")
                                      }
                                    }
                                    "during last epoch"
                                    { text(nice::int(last_epoch)) }
                                </div>
                            }
                        } else {
                            no_node()
                        }}
                        {if let Some(last_apr) = self.state.last_apr {
                            node! {
                                <div class="stats-row">
                                    { text(format!("APR: {:.2}%", 100.0*last_apr)) }
                                </div>
                            }
                        } else {
                            no_node()
                        }}
                        {if let Some(last_apy) = self.state.last_apy {
                            node! {
                                <div class="stats-row">
                                    { text(format!("APY: {:.2}%", 100.0*last_apy)) }
                                </div>
                            }
                        } else {
                            no_node()
                        }}

                        {if let Some(total_stake) = self.state.total_stake {
                            node! {
                                <div class="stats-row">
                                    "Total Stake "
                                    { text(nice::int(nice::dec(total_stake, 18))) }
                                </div>
                            }
                        } else {
                            no_node()
                        }}
                        <div class="stats-row">
                            { text("Last processed block")}
                            { text(nice::int(self.state.last_block)) }
                        </div>
                    </div>
                </div>
                { footer::render() }
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("MSG: {:?}", msg);
        Cmd::none()
    }
}
