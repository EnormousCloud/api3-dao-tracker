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
                </div>
                { footer::render() }
            </div>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}
