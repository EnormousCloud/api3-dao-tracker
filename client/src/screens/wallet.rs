use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::router::{link_eventlog, text_entry};
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, OnChainEvent};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::{U256, H160};

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

    pub fn render_event_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="c">"#"</th>
                <th class="c">"Date"</th>
                <th class="c" style="white-space:nowrap">"Block #"</th>
                <th class="l">"Event"</th>
            </tr>
        }
    }
    pub fn render_event(&self, _e: &OnChainEvent) -> Node<Msg> {
        div(vec![], vec![])
    }

    pub fn render_event_tr(&self, index: usize, e: &OnChainEvent) -> Node<Msg> {
        node! {
            <tr>
                <td class="c">{text(format!("{}.", index + 1))}</td>
                <td class="c darken dt">{text(nice::date(e.tm))}</td>
                <td class="c">{link_eventlog(self.state.chain_id, e.block_number, e.tx)}</td>
                <td class="l">{text_entry(&e.entry)}</td>
            </tr>
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
                        // TODO: paragraph
                        // TODO: rewards table
                        // TODO: delegation header
                    }
                    <h2>"Events Log"</h2>
                    {
                        match self.state.wallets_events.get(&self.addr) {
                            Some(w) => {
                                {
                                    if w.len() > 0 {
                                        div(vec![], vec![
                                            div(vec![class("desktop-only")], vec![
                                                table(vec![class("table events-table")],
                                                    vec![
                                                        thead(vec![], vec![ self.render_event_header() ]),
                                                        tbody(vec![], w.iter().enumerate().map(|(i, e)| self.render_event_tr(i, e)).collect::<Vec<Node<Msg>>>()),
                                                    ]
                                                )
                                            ]),
                                            div(vec![class("mobile-only")], vec![
                                                ol(vec
                                                    ![class("events-list")],
                                                    w.iter().enumerate().map(|(_, e)| self.render_event(e)).collect::<Vec<Node<Msg>>>()
                                                )
                                            ])
                                        ])
                                    } else {
                                        div(vec![class("events-empty")], vec![
                                            text("There were no wallet events in the DAO")
                                        ])
                                    }}
                               }
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

impl MetaProvider for Screen {
    fn meta(&self) -> PageMetaInfo {
        let (title, description) = match self.state.wallets.get(&self.addr) {
            Some(w) => {
                let total_votes = self.state.get_votes_total();
                let power = nice::pct_of(w.voting_power, total_votes, 18);
                (
                    format!("API3 DAO Member - {} has voting power of {}%", w.get_name(), power ),
                    format!("Explore API3 DAO voting power, shares and full staking history of {}. No wallet connection is needed", w.get_name()),
                )
            },
            None => (
                "API3 DAO Member was not found".to_owned(), 
                format!("Explore API3 DAO voting power, shares and full staking history of {} members. No wallet connection is needed", self.state.wallets.len()),
            ),
        };
        PageMetaInfo::new(&title, &description)
    }
}
