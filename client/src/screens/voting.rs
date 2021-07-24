use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::events::VotingAgent;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-voting">
                { header::render("/votings") }
                <div class="inner">
                    <h1>"API3 DAO Voting"</h1>
                    <h2>{ text(format!("{:?} {:?}", self.agent, self.vote_id )) }</h2>
                    {
                        match self.state.votings.get(&self.vote_ref) {
                            Some(v) => pre(
                                vec![class("votings-details")],
                                vec![text(format!("{}", serde_json::to_string_pretty(&v).unwrap()))]
                            ),
                            None => err_box("member wallet was not found")
                        }
                    }
                    <h2>"Events Log"</h2>
                    {
                        match self.state.votings_events.get(&self.vote_ref) {
                            Some(w) => ol(
                                vec![class("votings-events-list")],
                                w.iter().map(|v| {
                                    node!{
                                        <li class="event">
                                            <pre>
                                                { text(format!("{}", serde_json::to_string_pretty(&v).unwrap())) }
                                            </pre>
                                        </li>
                                    }
                                }).collect::<Vec<Node<Msg>>>()
                            ),
                            None => err_box("voting was not found")
                        }
                    }

                </div>
                { footer::render(&self.state) }
            </div>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}
