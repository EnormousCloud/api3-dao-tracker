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
            <div class="screen-wallets">
                { header::render("/wallets") }
                <div class="inner">
                    <h1>{text(format!("API3 DAO: {} Wallets", self.state.wallets.len()))}</h1>
                    {ol(vec
                        ![class("wallets-list")],
                        self.state.wallets.iter().map(|(k, _)| {
                            node!{
                                <li>
                                    <div class="wallet">
                                        { text(format!("{:?}", k)) }
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
