use crate::components::footer;
use crate::components::header;
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
                                { text(format!("{}", self.state.wallets.len())) }
                                " DAO members"
                            </a>
                        </div>
                        <div class="stats-row">
                            <a href="./votings">
                                { text(format!("{}", self.state.votings.len())) }
                                " DAO votings"
                            </a>
                        </div>
                        <div class="stats-row">
                            { text(format!("last block {}", self.state.last_block)) }
                        </div>
                        {if let Some(last_epoch) = self.state.last_epoch {
                            node! {
                                <div class="stats-row">
                                    { text(format!("last epoch {}", last_epoch)) }
                                </div>
                            }
                        } else {
                            div(vec![],vec![])
                        }}
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
