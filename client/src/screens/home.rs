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
pub enum Msg {
}

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
                        { text(format!("{} DAO members", self.state.wallets.len())) }
                        </div>
                        <div class="stats-row">
                        { text(format!("{} DAO votings", self.state.votings.len())) }
                        </div>
                        <div class="stats-row">
                        { text(format!("last event at block {}", self.state.last_block)) }
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
