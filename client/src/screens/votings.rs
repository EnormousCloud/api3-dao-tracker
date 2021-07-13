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

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-votings">
                { header::render("/votings") }
                <div class="inner">
                    <h1>"API3 DAO Votings"</h1>
                    {ol(vec
                        ![class("votings-list")],
                        self.state.votings.iter().map(|(_, voting)| {
                            node!{
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
