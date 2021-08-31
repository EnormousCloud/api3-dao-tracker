use crate::components::footer;
use crate::components::header;
use crate::components::panel;
use crate::nice;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::AppState;
use crate::router::link_address;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use web3::types::{U256, H160};

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

pub fn no_node<T>() -> Node<T> {
    div(vec![], vec![])
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {}

impl Screen {
    pub fn render_treasury(
        &self,
        divclass: &'static str,
        title: String,
        wallet: H160,
        tokens: Map<String, U256>,
        decimals: Map<String, usize>,
    ) -> Node<Msg> {
        panel::render(
            &title,
            divclass,
        div(vec![], vec![
            node!{
                <div style="text-align:center; margin-bottom: 20px;">
                    <span class="darken">{text(format!("{} ", wallet))}</span>
                    {link_address(self.state.chain_id, wallet, false)}
                </div>
            },
            div(vec![], tokens.iter().map(|(tokenname, value)| {
                    let default = 18 as usize;
                    let decimal = decimals.get(tokenname).unwrap_or(&default);
                    let cls = if *value == U256::from(0) { "darken big-title" } else { "big-title" };
                    node!{
                        <div style="margin-bottom: 20px">
                            <h3 style="text-align: center" class="cell-title">{text(tokenname.clone())} </h3>
                            <div style="text-align: center">
                                <strong class={cls} title={nice::amount(*value, *decimal)}>
                                    {text(nice::ceil(*value, *decimal))}
                                </strong>
                            </div>
                        </div>
                    }
                }).collect()),
            ]),
        )
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        let decimals = self.state.decimals.clone();
        node! {
            <div class="screen-treasury">
                { header::render("/treasury", &self.state) }
                <div class="inner">
                    <div class="centered">
                        <h1>"API3 DAO Treasury"</h1>
                        <p class="m20" style="text-align: center">
                            <span class="darken">
                                "API3 DAO currently operates 3 treasuries. Balances below are updated each hour."
                            </span>
                        </p>
                        <div style="height: 20px">" "</div>

                        {div(vec![class("dash-row")], self.state.treasuries.iter().map(|(_, t)| {
                            self.render_treasury("dash-col dash-col-3", t.name.clone(), t.wallet.clone(), t.balances.clone(), decimals.clone())
                        }).collect())}

                        <div style="height: 30px">" "</div>
                    </div>
                </div>
                { footer::render(&self.state) }
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("MSG: {:?}", msg);
        Cmd::none()
    }
}

impl MetaProvider for Screen {
    fn meta(&self) -> PageMetaInfo {
        let title = format!("API3 DAO Treasury - balances under DAO control");
        let description = "Explore API3 DAO Treasury balances. No wallet connection is needed";
        PageMetaInfo::new(&title, description)
    }
}
