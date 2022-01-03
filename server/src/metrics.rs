use client::nice;
use client::state::AppState;
use lazy_static::lazy_static;
use prometheus::{opts, register_gauge, register_int_gauge};
use prometheus::{Encoder, Gauge, IntGauge, Opts, Registry, TextEncoder};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    pub static ref CHAIN_ID_GAUGE: IntGauge =
        register_int_gauge!(opts!("chain_id", "Chain ID that is being syncing",)).unwrap();
    pub static ref WATCHING: IntGauge =
        register_int_gauge!(opts!("watching", "Whether the chain is being watched",)).unwrap();
    pub static ref BLOCK_START_GAUGE: IntGauge =
        register_int_gauge!(opts!("block_start", "Number of block that is being synced",)).unwrap();
    pub static ref BLOCK_END_GAUGE: IntGauge =
        register_int_gauge!(opts!("block_end", "Number of block that is being synced",)).unwrap();
}

lazy_static! {
    pub static ref NUM_ADDRESSES: IntGauge =
        register_int_gauge!(opts!("addresses", "Number of addresses in DAO",)).unwrap();
    pub static ref NUM_VOTINGS: IntGauge =
        register_int_gauge!(opts!("votings", "Number of votings in DAO",)).unwrap();
    pub static ref EPOCHS: IntGauge =
        register_int_gauge!(opts!("epochs", "Number of epochs",)).unwrap();
    pub static ref EPOCH_INDEX: IntGauge =
        register_int_gauge!(opts!("epoch_index", "Index of the current epoch",)).unwrap();
    pub static ref APR: Gauge =
        register_gauge!(opts!("apr", "Index of the current epoch",)).unwrap();
    pub static ref LAST_BLOCK: IntGauge =
        register_int_gauge!(opts!("last_block", "Index of the last block",)).unwrap();
    pub static ref GENESIS_APR: Gauge =
        register_gauge!(opts!("genesis_apr", "Genesis APR",)).unwrap();
    pub static ref MIN_APR: Gauge = register_gauge!(opts!("min_apr", "Min APR",)).unwrap();
    pub static ref MAX_APR: Gauge = register_gauge!(opts!("max_apr", "Max APR",)).unwrap();
    pub static ref STAKE_TARGET: Gauge =
        register_gauge!(opts!("stake_target", "Stake Target",)).unwrap();
    pub static ref UNSTAKE_WAIT_PERIOD: IntGauge =
        register_int_gauge!(opts!("unstake_wait_period", "Unstake Wait Period",)).unwrap();
    pub static ref REWARD_PERIOD: IntGauge =
        register_int_gauge!(opts!("reward_vesting_period", "Reward Vesting Period",)).unwrap();
    pub static ref CIRCULATING_SUPPLY: Gauge =
        register_gauge!(opts!("circulating_supply", "API3 Circulating Supply",)).unwrap();
    pub static ref TOTAL_SUPPLY: Gauge =
        register_gauge!(opts!("total_supply", "API3 Total Supply",)).unwrap();
    pub static ref LOCKED_GOV: Gauge =
        register_gauge!(opts!("locked_by_governance", "Locked By Governance",)).unwrap();
    pub static ref LOCKED_REWARDS: Gauge =
        register_gauge!(opts!("locked_rewards", "Locked Rewards",)).unwrap();
    pub static ref LOCKED_VESTINGS: Gauge =
        register_gauge!(opts!("locked_vestings", "Locked Vestings",)).unwrap();
    pub static ref TIME_LOCKED: Gauge =
        register_gauge!(opts!("time_locked", "Time Locked",)).unwrap();
    pub static ref TOTAL_LOCKED: Gauge =
        register_gauge!(opts!("total_locked", "Total Locked",)).unwrap();
    pub static ref SINCE_LAST_UPDATE: IntGauge = register_int_gauge!(opts!(
        "since_last_update",
        "Seconds since the last state update",
    ))
    .unwrap();
    pub static ref SINCE_LAST_VOTINGS: IntGauge = register_int_gauge!(opts!(
        "since_last_votings_read",
        "Seconds since the last votings read",
    ))
    .unwrap();
    pub static ref SINCE_LAST_ENS: IntGauge = register_int_gauge!(opts!(
        "since_last_ens_read",
        "Seconds since the last ENS read",
    ))
    .unwrap();
    pub static ref SINCE_LAST_TREASURIES: IntGauge = register_int_gauge!(opts!(
        "since_last_treasuries_read",
        "Seconds since the last treasuries read",
    ))
    .unwrap();
    pub static ref SINCE_LAST_CIRCULATION: IntGauge = register_int_gauge!(opts!(
        "since_last_circulation_read",
        "Seconds since the last circulation read",
    ))
    .unwrap();
}

pub fn handler(state: &AppState) -> String {
    let encoder = TextEncoder::new();
    let labels = HashMap::new();
    let sr = Registry::new_custom(Some("dao".to_string()), Some(labels)).unwrap();
    sr.register(Box::new(NUM_ADDRESSES.clone())).unwrap();
    sr.register(Box::new(NUM_VOTINGS.clone())).unwrap();
    sr.register(Box::new(CHAIN_ID_GAUGE.clone())).unwrap();
    sr.register(Box::new(WATCHING.clone())).unwrap();
    sr.register(Box::new(EPOCHS.clone())).unwrap();
    sr.register(Box::new(EPOCH_INDEX.clone())).unwrap();
    sr.register(Box::new(APR.clone())).unwrap();
    sr.register(Box::new(LAST_BLOCK.clone())).unwrap();
    // pool info
    sr.register(Box::new(GENESIS_APR.clone())).unwrap();
    sr.register(Box::new(MIN_APR.clone())).unwrap();
    sr.register(Box::new(MAX_APR.clone())).unwrap();
    sr.register(Box::new(STAKE_TARGET.clone())).unwrap();
    sr.register(Box::new(UNSTAKE_WAIT_PERIOD.clone())).unwrap();
    sr.register(Box::new(REWARD_PERIOD.clone())).unwrap();
    // circulating supply
    sr.register(Box::new(CIRCULATING_SUPPLY.clone())).unwrap();
    sr.register(Box::new(TOTAL_SUPPLY.clone())).unwrap();
    sr.register(Box::new(LOCKED_GOV.clone())).unwrap();
    sr.register(Box::new(LOCKED_REWARDS.clone())).unwrap();
    sr.register(Box::new(LOCKED_VESTINGS.clone())).unwrap();
    sr.register(Box::new(TIME_LOCKED.clone())).unwrap();
    sr.register(Box::new(TOTAL_LOCKED.clone())).unwrap();
    // seconds, passed since various events
    sr.register(Box::new(SINCE_LAST_UPDATE.clone())).unwrap();
    sr.register(Box::new(SINCE_LAST_VOTINGS.clone())).unwrap();
    sr.register(Box::new(SINCE_LAST_ENS.clone())).unwrap();
    sr.register(Box::new(SINCE_LAST_TREASURIES.clone()))
        .unwrap();
    sr.register(Box::new(SINCE_LAST_CIRCULATION.clone()))
        .unwrap();

    NUM_ADDRESSES.set(state.wallets.len() as i64);
    NUM_VOTINGS.set(state.votings.len() as i64);
    EPOCHS.set(state.epochs.len() as i64);
    EPOCH_INDEX.set(state.epoch_index as i64);
    APR.set(state.apr);
    LAST_BLOCK.set(state.last_block as i64);
    if let Some(pool) = &state.pool_info {
        GENESIS_APR.set(pool.genesis_apr);
        MIN_APR.set(pool.min_apr);
        MAX_APR.set(pool.max_apr);
        STAKE_TARGET.set(nice::dec(pool.stake_target, 18));
        UNSTAKE_WAIT_PERIOD.set(pool.unstake_wait_period as i64);
        REWARD_PERIOD.set(pool.reward_vesting_period as i64);
    }
    if let Some(ci) = &state.circulation {
        CIRCULATING_SUPPLY.set(nice::dec(ci.circulating_supply, 18));
        TOTAL_SUPPLY.set(nice::dec(ci.total_supply, 18));
        LOCKED_GOV.set(nice::dec(ci.locked_by_governance, 18));
        LOCKED_REWARDS.set(nice::dec(ci.locked_rewards, 18));
        LOCKED_VESTINGS.set(nice::dec(ci.locked_vestings, 18));
        TIME_LOCKED.set(nice::dec(ci.time_locked, 18));
        TOTAL_LOCKED.set(nice::dec(ci.total_locked, 18));
    }
    for (name, treasury) in &state.treasuries {
        for (coin, balance) in &treasury.balances {
            let decimals = state.decimals.get(coin).unwrap();
            let gauge_opts = Opts::new("balance", "treasury balance")
                .const_label("coin", coin.as_str())
                .const_label("treasury", name.as_str());
            let gauge = Gauge::with_opts(gauge_opts).unwrap();
            gauge.set(nice::dec(balance, *decimals));
            sr.register(Box::new(gauge.clone())).unwrap();
        }
    }
    let now = SystemTime::now();
    let secs_now: i64 = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    if let Some(the_last) = &state.the_last {
        SINCE_LAST_UPDATE.set(match the_last.update {
            Some(sys_time) => secs_now - sys_time.timestamp(),
            None => -1,
        });
        SINCE_LAST_VOTINGS.set(match the_last.votings {
            Some(sys_time) => secs_now - sys_time.timestamp(),
            None => -1,
        });
        SINCE_LAST_ENS.set(match the_last.ens {
            Some(sys_time) => secs_now - sys_time.timestamp(),
            None => -1,
        });
        SINCE_LAST_TREASURIES.set(match the_last.treasuries {
            Some(sys_time) => secs_now - sys_time.timestamp(),
            None => -1,
        });
        SINCE_LAST_CIRCULATION.set(match the_last.circulation {
            Some(sys_time) => secs_now - sys_time.timestamp(),
            None => -1,
        });
    }

    let mut buffer = Vec::<u8>::new();
    encoder.encode(&sr.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}

pub fn syncing() -> String {
    let encoder = TextEncoder::new();
    let labels = HashMap::new();
    let sr = Registry::new_custom(Some("sync".to_string()), Some(labels)).unwrap();
    sr.register(Box::new(CHAIN_ID_GAUGE.clone())).unwrap();
    sr.register(Box::new(WATCHING.clone())).unwrap();
    sr.register(Box::new(BLOCK_START_GAUGE.clone())).unwrap();
    sr.register(Box::new(BLOCK_END_GAUGE.clone())).unwrap();
    let mut buffer = Vec::<u8>::new();
    encoder.encode(&sr.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
