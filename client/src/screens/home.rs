use crate::components::footer;
use crate::components::header;
use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::U256;

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
    pub fn rewards_coeff(&self) -> f64 {
        match &self.state.pool_info {
            Some(x) => x.clone().rewards_coeff,
            None => 1f64
        }
    }
    pub fn render_supply(&self) -> Node<Msg> {
        let total_staked = self.state.get_staked_total();
        let stake_target: U256 = match &self.state.pool_info {
            Some(x) => x.clone().stake_target,
            None => U256::from(0)
        };
        match &self.state.circulation {
            Some(c) => node!{
                <div>
                    <h3> "API3 Circulating Supply" </h3>
                    <strong title={nice::amount(c.circulating_supply, 18)}>
                        {text(nice::ceil(c.circulating_supply, 18))}
                        " tokens"
                    </strong>
                    <h3> "Total Locked" </h3>
                    <strong title={nice::amount(c.total_locked, 18)}>
                        {text(nice::ceil(c.total_locked, 18))}
                        " tokens"
                    </strong>
                    <div class="dash-row">
                        <div class="dash-col dash-col-2">
                            <h3> "Staked in DAO" </h3>
                            <strong title={nice::amount(total_staked, 18)}>
                                {text(nice::ceil(total_staked, 18))}
                                " tokens"
                            </strong>
                        </div>
                        <div class="dash-col dash-col-2">
                            <h3> "Staking Target" </h3>
                            <strong title={nice::amount(stake_target, 10)}>
                                {text(nice::ceil(stake_target, 10))}
                                " tokens"
                            </strong>
                        </div>
                    </div>
                </div>
            },
            None => div(vec![class("error")],vec![text("no supply info")]),
        }
    }

    pub fn render_locked(&self) -> Node<Msg> {
        match &self.state.circulation {
            Some(c) => node!{
                <div class="dash-row">
                    <div class="dash-col dash-col-4">
                        <h3> "Locked by governance" </h3>
                        <strong title={nice::amount(c.locked_by_governance, 18)}>
                            {text(nice::ceil(c.locked_by_governance, 18))}
                            " tokens"
                        </strong>
                    </div>
                    <div class="dash-col dash-col-4">
                        <h3> "Locked vestings" </h3>
                        <strong title={nice::amount(c.locked_vestings, 18)}>
                            {text(nice::ceil(c.locked_vestings, 18))}
                            " tokens"
                        </strong>
                    </div>
                    <div class="dash-col dash-col-4">
                        <h3> "Locked rewards" </h3>
                        <strong title={nice::amount(c.locked_rewards, 18)}>
                            {text(nice::ceil(c.locked_rewards, 18))}
                            " tokens"
                        </strong>
                    </div>
                    <div class="dash-col dash-col-4">
                        <h3> "Time Locked" </h3>
                        <strong title={nice::amount(c.time_locked, 18)}>
                            {text(nice::ceil(c.time_locked, 18))}
                            " tokens"
                        </strong>
                    </div>
                </div>
            },
            None => div(vec![class("error")],vec![text("no supply info")]),
        }
    }
    
    pub fn render_contracts(&self) -> Node<Msg> {
        match &self.state.circulation {
            Some(c) => node!{
                <div>
                    <ul>
                        <li>
                            <label>"API3 Pool contract address: "</label>
                            <strong>{text(format!("{:?}", c.addr_pool))}</strong>
                        </li>
                        <li>
                            <label>"API3 Token contract address: "</label>
                            <strong>{text(format!("{:?}", c.addr_token))}</strong>
                        </li>
                        <li>
                            <label>"Time-lock manager address:"</label>
                            <strong>{text(format!("{:?} ", c.addr_time_lock))}</strong>
                        </li>
                        <li>
                            <label>"Primary Treasury address: "</label>
                            <strong>{text(format!("{:?}", c.addr_primary_treasury))}</strong>
                        </li>
                        <li>
                            <label>"Secondary Treasury address: "</label>
                            <strong>{text(format!("{:?}", c.addr_secondary_treasury))}</strong>
                        </li>
                        <li>
                            <label>"V1 Treasury address: "</label>
                            <strong>{text(format!("{:?}", c.addr_v1_treasury))}</strong>
                        </li>
                    </ul>
                </div>
            }, 
            None => div(vec![class("error")],vec![text("no circulation info")]),
        }
    }

    pub fn current_epoch(&self) -> Node<Msg> {
        node!{
            <div>
                <h2>
                    "current epoch "
                    {text(format!("{}", self.state.epoch_index))}
                </h2>
                <div class="stats-row">
                    "Epoch APR: "
                    <strong>
                        { text(format!("{:.2}%", 100.0*self.state.apr)) }
                    </strong>
                </div>
                <div class="stats-row">
                    "Epoch Rewards: "
                    <strong>
                        { text(format!("{:.2}%", 100.0*self.state.apr*self.rewards_coeff())) }
                    </strong>
                </div>
            </div>
        }
    }

    pub fn render_epoch(&self, epoch: u64) -> Node<Msg> {
        let prev_epoch = self.state.epoch_index - epoch;
        if let Some(ep) = self.state.epochs.get(&prev_epoch) {
            node!{
                <div>
                    <h2>
                        "previous epoch "
                        {text(format!("{}", ep.index))}
                    </h2>
                    <div class="stats-row">
                        "Epoch APR:"
                        <strong>
                            { text(format!("{:.2}%", 100.0*ep.apr)) }
                        </strong>
                    </div>
                    <div class="stats-row">
                        "Epoch Rewards:"
                        <strong>
                            { text(format!("{:.2}%", 100.0*ep.apr*self.rewards_coeff())) }
                        </strong>
                    </div>
                    <div class="stats-row">
                        <strong>
                            { text(nice::int(nice::dec(ep.minted, 18))) }
                        </strong>
                        " API3 tokens minted"
                    </div>
                    <div class="stats-row">
                        "Staked at the end of epoch: "
                        <strong title={nice::amount(ep.total, 18)}>
                            { text(nice::ceil(ep.total, 18)) }
                        </strong>
                    </div>
                </div>
            }

        } else {
            text("-")
        }
    }
}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-home">
                { header::render("") }
                <div class="inner">
                    <div class="centered">
                        <h1>"API3 DAO Tracker"</h1>
                        <h2>"on-chain analytics"</h2>

                        <div class="dash-row">
                            <div class="dash-col dash-col-2">
                                <a href="./wallets">
                                    { text(nice::int(self.state.wallets.len())) }
                                    " DAO members"
                                </a>
                            </div>
                            <div class="dash-col dash-col-2">
                                <a href="./votings">
                                    { text(nice::int(self.state.votings.len())) }
                                    " DAO votings"
                                </a>
                            </div>
                        </div>
                        <div class="dash-row">
                            <div class="dash-col dash-col-4">
                                {self.current_epoch()}
                            </div>
                            <div class="dash-col dash-col-4">
                                {self.render_epoch(1)}
                            </div>
                        </div>

                        <div class="dash-row">
                            <div class="dash-col dash-col-2">
                                {self.render_supply()}
                            </div>
                            <div class="dash-col dash-col-2">
                                {self.render_contracts()}
                            </div>
                        </div>
                        {self.render_locked()}
                        <div class="stats-row the-last">
                            { text("Last block with events ")}
                            { text(nice::int(self.state.last_block)) }
                        </div>
                    </div>
                </div>
                { footer::render() }
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("MSG: {:?}", msg);
        Cmd::none()
    }
}
