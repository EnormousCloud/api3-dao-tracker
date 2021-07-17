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
        let prev_epoch = self.state.epoch_index - 1;
        let last_epoch = self.state.epochs.get(&prev_epoch);
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
                        <div>
                            <div class="stats-row">
                                "Current APR"
                                <strong>
                                    { text(format!("{:.2}%", 100.0*self.state.apr)) }
                                </strong>
                            </div>
                            <div class="stats-row">
                                "Current APY"
                                <strong>
                                    { text(format!("{:.2}%", 100.0*self.state.apy)) }
                                </strong>
                            </div>
                        </div>

                        {if let Some(last_epoch) = last_epoch {
                            node! {
                                <div>
                                    <div class="stats-row">
                                        <strong>
                                            { text(nice::int(nice::dec(last_epoch.minted, 18))) }
                                        </strong>
                                        "API3 tokens minted during last epoch"
                                        <strong>
                                            { text(nice::int(last_epoch.index)) }
                                        </strong>
                                    </div>
                                    <div class="stats-row">
                                        "Last Epoch APR:"
                                        <strong>
                                            { text(format!("{:.2}%", 100.0*last_epoch.apr)) }
                                        </strong>
                                    </div>
                                    <div class="stats-row">
                                        "Last Epoch APY:"
                                        <strong>
                                            { text(format!("{:.2}%", 100.0*last_epoch.apy)) }
                                        </strong>
                                    </div>
                                    <div class="stats-row">
                                        "Last Epoch Stake "
                                        <strong>
                                            { text(nice::int(nice::dec(last_epoch.total, 18))) }
                                        </strong>
                                    </div>
                                </div>
                            }
                        } else {
                            no_node()
                        }}
                        <div class="stats-row">
                            { text("Last block with events")}
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
