use crate::components::footer;
use crate::components::header;
use crate::events::VotingAgent;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    // ID of the voting
    pub vote_id: u64,
    // agent of the voting
    pub agent: VotingAgent,
    /// server side state
    pub state: AppState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-voting">
                { header::render("/votings") }
                <div class="inner">
                    <h1>"API3 DAO Voting"</h1>
                    <h2>{ text(format!("{:?} {:?}", self.agent, self.vote_id )) }</h2>
                </div>
                { footer::render() }
            </div>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}
