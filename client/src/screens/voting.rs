use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::events::{self, VotingAgent};
use crate::screens::meta::{MetaProvider, PageMetaInfo};
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

impl MetaProvider for Screen {
    fn meta(&self) -> PageMetaInfo {
        let description =
            "Explore API3 DAO full voting history. No wallet connection is needed".to_owned();
        let title = match self.state.votings.get(&self.vote_ref) {
            Some(v) => format!(
                "API3 DAO {} Voting History",
                if v.primary { "Primary" } else { "Secondary" }
            ),
            None => "API3 DAO Voting was not found".to_owned(),
        };
        PageMetaInfo::new(&title, &description)
    }
}
