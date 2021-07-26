use crate::components::footer;
use crate::components::header;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, Voting};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

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

    pub fn render_voting(&self, voting: &Voting) -> Node<Msg> {
        node! {
            <li>
                <div class="voting">
                    <a href={format!("votings/{}", voting.key()) }>
                        { text(format!("{}: {:?}",
                            if voting.primary {
                                "Primary"
                            } else {
                                "Secondary"
                            }, voting.metadata)) }
                    </a>
                </div>
            </li>
        }
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-votings">
                { header::render("/votings") }
                <div class="inner">
                    <h1>"API3 DAO Votings"</h1>
                    {if self.state.votings.len() > 0 {
                        ol(vec
                            ![class("votings-list")],
                            self.state.votings.iter().map(|(_, voting)| self.render_voting(voting)).collect::<Vec<Node<Msg>>>()
                        )
                    } else {
                        div(vec![class("votings-empty")], vec![
                            text("There were no votings in the DAO so far")
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
        let title = "API3 DAO Tracker - Full Votings History";
        let description = format!(
            "Explore {} votings history of API3 DAO. No wallet connection is needed",
            self.state.votings.len()
        );
        PageMetaInfo::new(&title, &description)
    }
}
