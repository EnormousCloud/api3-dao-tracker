use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::router::link_eventlog;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, Epoch};
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

    pub fn rewards_coeff(&self) -> f64 {
        match &self.state.pool_info {
            Some(x) => x.clone().rewards_coeff,
            None => 1f64,
        }
    }

    pub fn release_offset(&self) -> u64 {
        match &self.state.pool_info {
            Some(pool_info) => pool_info.reward_vesting_period * pool_info.epoch_length,
            None => 0u64,
        }
    }

    pub fn render_epoch_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="c">"Epoch"</th>
                <th class="c">"Block"</th>
                <th class="c">"Rewards Date"</th>
                <th class="r">"APR"</th>
                <th class="r">"Rewards"</th>
                <th class="r">"Members"</th>
                <th class="r">"Staked"</th>
                <th class="r">"Minted"</th>
                <th class="c">"Release Date"</th>
            </tr>
        }
    }

    pub fn render_epoch_tr(&self, ep: &Epoch) -> Node<Msg> {
        node! {
            <tr>
                <td class="c">{text(nice::int(ep.index))}</td>
                <td class="c">{link_eventlog(self.state.chain_id, ep.block_number, ep.tx)}</td>
                <td class="c">{ text(nice::date(ep.tm)) }</td>
                <td class="r darken">{ text(format!("{:.2}%", 100.0*ep.apr)) }</td>
                <td class="r accent">{ text(format!("{:.4}%", 100.0*ep.apr*self.rewards_coeff() / 52.0)) }</td>
                <td class="r darken">{ text(nice::int(ep.stake.len())) }</td>
                <td class="r darken" title={nice::amount(ep.total, 18)}>{ text(nice::ceil(ep.total, 18)) }</td>
                <td class="r accent" title={nice::amount(ep.minted, 18)}>{ text(nice::ceil(ep.minted, 18)) }</td>
                <td class="c">{ text(nice::date(ep.tm + self.release_offset())) }</td>
            </tr>
        }
    }

    pub fn render_epoch(&self, ep: &Epoch) -> Node<Msg> {
        node! {
            <li>
                <div class="epoch">
                    <div class="cell-title">
                        <span class="darken">"Epoch #"</span>
                        {text(nice::int(ep.index))}
                    </div>
                    <div class="stats-row darken cell-title">
                        { text(nice::date(ep.tm)) }
                    </div>
                    <div class="stats-row">
                        <span class="darken">"APR: "</span>
                        <strong class="big-title">
                            { text(format!("{:.2}%", 100.0*ep.apr)) }
                        </strong>
                    </div>
                    <div class="stats-row m20">
                        <span class="darken cell-title">"Epoch Rewards: "</span>
                        <strong class="accent">
                            { text(format!("{:.4}%", 100.0*ep.apr*self.rewards_coeff() / 52.0)) }
                        </strong>
                    </div>
                    <div class="stats-row">
                        <div class="darken cell-title">"Staked at the end of epoch: "</div>
                        <strong title={nice::amount(ep.total, 18)}>
                            { text(nice::ceil(ep.total, 18)) }
                        </strong>
                    </div>
                    <div class="padded">
                        <div class="stats-row cell-title">
                            <strong>
                                { text(nice::int(nice::dec(ep.minted, 18))) }
                            </strong>
                            <span class="darken">
                                " API3 tokens were minted"
                            </span>
                        </div>
                        <div class="stats-row cell-title">
                            <strong>
                                { text(nice::int(ep.stake.len())) }
                            </strong>
                            <span class="darken">
                                " members were rewarded"
                            </span>
                        </div>
                        <div class="stats-row cell-title">
                            <span class="darken">
                                "Will be released: "
                            </span>
                            <strong>
                                {text(nice::date(ep.tm + self.release_offset()))}
                            </strong>
                        </div>
                    </div>
                </div>
            </li>
        }
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        let minted = self.state.get_minted_total();
        node! {
            <div class="screen-rewards">
                { header::render("/rewards") }
                <div class="inner">
                    <div class="centered">
                        <h1>"API3 DAO Staking Rewards History"</h1>
                        <p style="text-align: center">
                            <span class="darken"> "API3 DAO minted " </span>
                            <strong title={nice::amount(minted, 18)}>
                                { text(nice::ceil(minted, 18)) }
                            </strong>
                            <span class="darken"> " API3 tokens as staking rewards for its members." </span>
                        </p>
                        {if self.state.epochs.len() > 0 {
                            div(vec![], vec![
                                div(vec![class("desktop-only")], vec![
                                    table(vec
                                        ![class("table epochs-table")],
                                        vec![
                                            thead(vec![], vec![ self.render_epoch_header() ]),
                                            tbody(vec![], self.state.epochs.iter().map(|(_, epoch)| self.render_epoch_tr(epoch)).collect::<Vec<Node<Msg>>>()),
                                        ]
                                    )
                                ]),
                                div(vec![class("mobile-only")], vec![
                                    ol(vec
                                        ![class("epochs-list")],
                                        self.state.epochs.iter().map(|(_, epoch)| self.render_epoch(epoch)).collect::<Vec<Node<Msg>>>()
                                    )
                                ])
                            ])
                        } else {
                            div(vec![class("epochs-empty")], vec![
                                text("There were no rewards distributions yet")
                            ])
                        }}
                    </div>
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
        let minted = self.state.get_minted_total();
        let title = "API3 DAO Staking Rewards History";
        let description = format!(
            "Explore API3 DAO staking rewards - {} API3 tokens minted as rewards for DAO members. No wallet connection needed.",
            nice::ceil(minted, 18)
        );
        PageMetaInfo::new(title, &description)
    }
}
