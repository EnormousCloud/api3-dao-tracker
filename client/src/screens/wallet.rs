use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::H160;

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    // address of the wallet
    pub addr: H160,
    /// server side state
    pub state: AppState,
}

impl Screen {
    pub fn new(state: AppState, addr: &H160) -> Self {
        Self {
            state: state.clone(),
            addr: addr.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-wallet">
                { header::render("/wallets") }
                <div class="inner">
                    <h1>"API3 DAO Wallet"</h1>
                    <h2>{ text(format!("{:?}", self.addr )) }</h2>
                    {
                        match self.state.wallets.get(&self.addr) {
                            Some(w) => pre(
                                vec![class("wallets-details")],
                                vec![text(format!("{}", serde_json::to_string_pretty(&w).unwrap()))]
                            ),
                            None => err_box("member wallet was not found")
                        }
                    }
                    <h2>"Events Log"</h2>
                    {
                        match self.state.wallets_events.get(&self.addr) {
                            Some(w) => ol(
                                vec![class("wallets-events-list")],
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
                            None => err_box("member wallet was not found")
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
