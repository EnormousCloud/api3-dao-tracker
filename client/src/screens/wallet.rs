use crate::components::err_box;
use crate::components::footer;
use crate::components::header;
use crate::components::panel;
use crate::eventsnode::entry_node;
use crate::nice;
use crate::router::{link_address, link_eventlog, link_wallet};
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, Epoch, OnChainEvent, Wallet};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::{H160, U256};

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
                <th class="c" style="white-space:nowrap; width:133px">"Block #"</th>
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
                <td class="l entry darken">{entry_node(&e.entry, self.addr, &self.state)}</td>
            </tr>
        }
    }

    pub fn render_delegation(&self, _addr: &H160, _shares: &U256) -> Node<Msg> {
        div(vec![], vec![])
    }

    pub fn render_delegation_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="l">"Delegated by Others"</th>
                <th class="r" style="width: 200px">"Shares"</th>
            </tr>
        }
    }

    pub fn render_delegation_tr(&self, addr: &H160, shares: &U256) -> Node<Msg> {
        node! {
            <tr>
                <td class="l">{link_wallet(&self.state, addr.clone())}</td>
                <td class="r" title={nice::amount(*shares, 18)}>{text(nice::ceil(*shares,18))}</td>
            </tr>
        }
    }

    pub fn render_delegation_info(&self, w: &Wallet) -> Node<Msg> {
        if w.delegated.len() == 0 {
            return text("");
        }
        div(
            vec![],
            vec![
                div(
                    vec![class("desktop-only")],
                    vec![table(
                        vec![class("table delegations-table")],
                        vec![
                            thead(vec![], vec![self.render_delegation_header()]),
                            tbody(
                                vec![],
                                w.delegated
                                    .iter()
                                    .map(|(addr, shares)| self.render_delegation_tr(addr, shares))
                                    .collect::<Vec<Node<Msg>>>(),
                            ),
                        ],
                    )],
                ),
                div(
                    vec![class("mobile-only")],
                    vec![ol(
                        vec![class("delegations-list")],
                        w.delegated
                            .iter()
                            .map(|(addr, shares)| self.render_delegation(&addr, &shares))
                            .collect::<Vec<Node<Msg>>>(),
                    )],
                ),
            ],
        )
    }

    pub fn render_wallet_info(&self, w: &Wallet) -> Node<Msg> {
        let labels = self.state.get_labels(w);
        let total_votes = self.state.get_votes_total();
        let pct = format!("{}%", nice::pct3_of(w.voting_power, total_votes, 18));

        let mut out: Vec<Node<Msg>> = vec![
            // text(format!("{}", serde_json::to_string_pretty(&w).unwrap())),
        ];
        if let Some(d) = &w.delegates {
            out.push(node! {
                <div class="delegates-all">
                    "This member delegates all his voting power to "
                    {link_wallet(&self.state, d.address.clone())}
                </div>
            });
        };
        out.push(node!{
            <div class="dash-row">
                {panel::render(
                    "Voting Power",
                    "dash-col dash-col-2",
                    node! {
                        <div>
                            <div style="text-align: center">
                                <strong class="accent" title={nice::amount(w.voting_power, 18)}>
                                    {text(nice::ceil(w.voting_power, 18))}
                                    <span class="darken">" shares"</span>
                                </strong>
                            </div>

                            <div style="padding-top: 30px; text-align: center">
                                <strong class="big-title">
                                    {if pct != "000.0%" {
                                        text(pct)
                                    } else {
                                        text("-")
                                    }}
                                </strong>
                            </div>
                            <div class="darken" style="padding-bottom: 30px; text-align: center">
                                "of total voting power"
                            </div>

                            <div style="padding-bottom: 30px; text-align: center">
                                <strong title={nice::amount(w.shares, 18)}>
                                    <span class="darken">" Owning "</span>
                                    {text(nice::ceil(w.shares, 18))}
                                    <span class="darken">" shares"</span>
                                </strong>
                            </div>
                        </div>
                    }
                )}
                <div class="dash-col dash-col-2">
                    <div class="dash-row">
                        <div class="dash-col dash-col-3 cell-t">
                            <h3 class="cell-title">"Deposited"</h3>
                            <strong class="big-title" title={nice::amount(w.deposited, 18)}>
                                {text(nice::ceil(w.deposited, 18))}
                            </strong>
                        </div>
                        <div class="dash-col dash-col-3 cell-t">
                            <h3 class="cell-title">"Withdrawn"</h3>
                            <strong class="big-title" title={nice::amount(w.withdrawn, 18)}>
                                {text(nice::ceil(w.withdrawn, 18))}
                            </strong>
                        </div>
                        <div class="dash-col dash-col-3 cell-t">
                            <h3 class="cell-title">"Locked Rewards"</h3>
                            <strong class="big-title accent" title={nice::amount(w.rewards, 18)}>
                                {text(nice::ceil(w.rewards, 18))}
                            </strong>
                        </div>
                    </div>

                    <h3 class="cell-title border-t" style="padding-top:30px;"> "Member Classification" </h3>
                    {ul(vec![class("badges")], labels.iter().map(|v| {
                        let title = format!("{}", v.title);
                        node! {
                            <li>
                                <span class={format!("badge {}", v.class)} title={title.clone()}>{text(v.text.clone().as_str())}</span>
                                <span class="darken">
                                    " - "
                                    {text(title)}
                                </span>
                            </li>
                        }
                    }).collect::<Vec<Node<Msg>>>())}

                </div>
            </div>
        });

        div(vec![], out)
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
                <th class="r">"Total"</th>
                <th class="r">"Minted"</th>
                <th class="r">"Reward"</th>
                <th class="r" title="Own shares + Rewards">"Shares"</th>
                <th class="r">"Received"</th>
                <th class="c">"Release Date"</th>
            </tr>
        }
    }

    pub fn render_epoch_tr(&self, ep: &Epoch, addr: &H160) -> Node<Msg> {
        let staked = self.state.get_staked_for_epoch(addr, ep.index);
        let reward = self.state.get_rewards_for_epoch(addr, ep.index);
        node! {
            <tr>
                <td class="c">{text(nice::int(ep.index))}</td>
                <td class="c">{link_eventlog(self.state.chain_id, ep.block_number, ep.tx)}</td>
                <td class="c">{ text(nice::date(ep.tm)) }</td>
                <td class="r darken">{ text(format!("{:.2}%", 100.0*ep.apr)) }</td>
                <td class="r darken" title={nice::amount(ep.total, 18)}>{ text(nice::ceil(ep.total, 18)) }</td>
                <td class="r darken" title={nice::amount(ep.minted, 18)}>{ text(nice::ceil(ep.minted, 18)) }</td>
                <td class="r accent">{ text(format!("{:.4}%", 100.0*ep.apr*self.rewards_coeff() / 52.0)) }</td>
                <td class="r" title={nice::amount(staked, 18)}>{ text(nice::ceil(staked, 18)) }</td>
                <td class="r accent" title={nice::amount(reward, 18)}>{ text(nice::ceil(reward, 18)) }</td>
                <td class="c">{ text(nice::date(ep.tm + self.release_offset())) }</td>
            </tr>
        }
    }

    pub fn render_epoch(&self, ep: &Epoch, _addr: &H160) -> Node<Msg> {
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

    pub fn render_rewards(&self, w: &Wallet) -> Node<Msg> {
        if self.state.epochs.len() > 0 {
            div(
                vec![],
                vec![
                    div(
                        vec![class("desktop-only")],
                        vec![table(
                            vec![class("table epochs-table")],
                            vec![
                                thead(vec![], vec![self.render_epoch_header()]),
                                tbody(
                                    vec![],
                                    self.state
                                        .epochs
                                        .iter()
                                        .map(|(_, epoch)| self.render_epoch_tr(epoch, &w.address))
                                        .collect::<Vec<Node<Msg>>>(),
                                ),
                            ],
                        )],
                    ),
                    div(
                        vec![class("mobile-only")],
                        vec![ol(
                            vec![class("epochs-list")],
                            self.state
                                .epochs
                                .iter()
                                .map(|(_, epoch)| self.render_epoch(epoch, &w.address))
                                .collect::<Vec<Node<Msg>>>(),
                        )],
                    ),
                ],
            )
        } else {
            div(
                vec![class("epochs-empty")],
                vec![text("There were no rewards distributions yet")],
            )
        }
    }
}

pub fn get_wallet_title(w: &Wallet) -> Node<Msg> {
    if let Some(ens) = &w.ens {
        return span(
            vec![],
            vec![
                text("API3 DAO Member "),
                strong(
                    vec![styles([("color", "var(--color-accent)")])],
                    vec![text(ens)],
                ),
            ],
        );
    }
    text("API3 DAO Member")
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-wallet">
                { header::render("/wallets", &self.state) }
                <div class="inner">
                    {
                        match self.state.wallets.get(&self.addr) {
                            Some(w) => div(
                                vec![class("wallets-details")],
                                vec![
                                    h1(vec![], vec![get_wallet_title(&w)]),
                                    h2(vec![styles([("text-align", "center")])], vec![
                                        text(format!("{:?} ", w.address)),
                                        link_address(self.state.chain_id, w.address, false),
                                    ]),
                                    self.render_wallet_info(&w),
                                    h2(vec![styles([("text-align", "center")])], vec![text("User Rewards")]),
                                    self.render_rewards(&w),
                                    self.render_delegation_info(&w),
                                ]
                            ),
                            None => err_box("member wallet was not found")
                        }
                    }
                    <h2 style="text-align:center">"User Events History"</h2>
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
