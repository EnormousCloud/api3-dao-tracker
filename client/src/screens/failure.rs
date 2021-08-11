use crate::components::footer;
use crate::components::header;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// failure message
    pub msg: String,
    /// server side state
    pub state: AppState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-failure">
                { header::render("", &self.state) }
                <div class="inner">
                    <h1>{ text(self.msg.as_str()) }</h1>
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
        PageMetaInfo::new(
            "API3 DAO Tracker - Error",
            "API3 DAO tracker watches API3 token supply, on-chain DAO events, displays history of each participant and staking rewards. No wallet connection is needed.",
        )
    }
}
