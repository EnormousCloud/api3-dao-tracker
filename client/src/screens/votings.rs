use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::screens::meta::{MetaProvider, PageMetaInfo};
use crate::state::{AppState, Voting};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::U256;

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

    pub fn render_voting_header(&self) -> Node<Msg> {
        node! {
            <tr>
                <th class="c">"#"</th>
                <th class="c">"Start Date"</th>
                <th class="c">"Type"</th>
                <th class="l">"Title"</th>
                <th class="r">"For"</th>
                <th class="r">"Against"</th>
                <th class="r">"Executed"</th>
            </tr>
        }
    }
    pub fn render_voting_tr(&self, index: usize, v: &Voting) -> Node<Msg> {
        let pct_required = if v.primary { 50u64 } else { 15u64 };
        let required = v.votes_total * U256::from(pct_required) / U256::from(100);
        let pct_yes = nice::pct3_of(v.voted_yes, v.votes_total, 18);
        let pct_no = nice::pct3_of(v.voted_no, v.votes_total, 18);
        let class_yes = if v.voted_yes > required {
            "r accent"
        } else {
            "r"
        };
        let class_no = if v.voted_no > required {
            "r warning"
        } else {
            "r"
        };
        node! {
            <tr>
                <td class="c">{text(format!("{}.", index + 1))}</td>
                <td class="c darken dt">{text(nice::date(v.tm))}</td>
                <td class="c">{
                    if v.primary {
                        span(vec![class("badge badge-primary")], vec![text("Primary")])
                    } else {
                        span(vec![class("badge badge-secondary")], vec![text("Secondary")])
                    }
                }</td>
                <td class="l">
                    <div>
                        <a href={format!("votings/{}", v.key()) }>
                            <strong>{text(v.title.clone())}</strong>
                        </a>
                    </div>
                    <div>
                        <small>{text(v.trigger_str())}</small>
                    </div>
                </td>
                <td class={class_yes}>{text(pct_yes)}"%"</td>
                {
                    if pct_no != "0.000" {
                        node!{ <td class={class_no}>{text(pct_no)}"%"</td> }
                    } else {
                        node!{ <td class="r"></td> }
                    }
                }
                <td class="r">{
                    if v.executed {
                        span(vec![class("badge")], vec![text("Executed ")])
                    } else {
                        span(vec![class(class_no)], vec![text("NO ")])
                    }
                }</td>
            </tr>
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
        let sorted: Vec<Voting> = self.state.votings.values().cloned().collect();
        node! {
            <div class="screen-votings">
                { header::render("/votings", &self.state) }
                <div class="inner">
                    <h1>"API3 DAO Votings"</h1>
                    {if self.state.votings.len() > 0 {
                        div(vec![], vec![
                            div(vec![class("desktop-only")], vec![
                                table(vec
                                    ![class("table votings-table")],
                                    vec![
                                        thead(vec![], vec![ self.render_voting_header() ]),
                                        tbody(vec![], sorted.iter().enumerate().map(|(i, v)| self.render_voting_tr(i, v)).collect::<Vec<Node<Msg>>>()),
                                    ]
                                )
                            ]),
                            div(vec![class("mobile-only")], vec![
                                ol(vec
                                    ![class("votings-list")],
                                    sorted.iter().enumerate().map(|(_, v)| self.render_voting(v)).collect::<Vec<Node<Msg>>>()
                                )
                            ])
                        ])
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
