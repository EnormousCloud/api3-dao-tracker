use client::nice;
use client::state::{Api3Circulation, Api3PoolInfo, VotingStaticData};
use tracing::warn;
use web3::contract::{Contract, Options};
use web3::types::{H160, U256};

#[derive(Debug)]
pub struct Convenience<T>
where
    T: web3::Transport,
{
    contract: Contract<T>,
}

impl<T: web3::Transport> Convenience<T> {
    pub fn new(web3: &web3::Web3<T>, address: H160) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contract/api3_convenience.abi.json"),
        )
        .expect("fail contract::from_json(api3_convenience.abi.json)");
        Convenience { contract: contract }
    }

    // we cannot get that from events properly
    pub async fn get_voting_static_data(
        &self,
        primary: bool,
        addr: H160,
        vote_id: u64,
    ) -> Option<VotingStaticData> {
        let t: u8 = if primary { 0 } else { 1 };
        let vote = U256::from(vote_id);
        let (
            start_date,
            support_required,
            min_quorum,
            voting_power,
            script,
            user_voting_power,
            discussion_url,
        ): (
            Vec<U256>,
            Vec<U256>,
            Vec<U256>,
            Vec<U256>,
            Vec<Vec<u8>>,
            Vec<U256>,
            Vec<String>,
        ) = match self //
            .contract
            .query(
                "getStaticVoteData",
                (t, addr, vec![vote]),
                None,
                Options::default(),
                None,
            )
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        };
        Some(VotingStaticData {
            start_date: start_date[0].as_u64(),
            support_required: nice::dec(support_required[0], 14) * 0.0001, // typically 0.5
            min_quorum: nice::dec(min_quorum[0], 14) * 0.001, //typically 0.15 for secondary
            voting_power: voting_power[0],
            script: script[0].clone(),
            user_voting_power_at: user_voting_power[0],
            discussion_url: discussion_url[0].clone(),
        })
    }
}

#[derive(Debug)]
pub struct Pool<T>
where
    T: web3::Transport,
{
    contract: Contract<T>,
}

impl<T: web3::Transport> Pool<T> {
    pub fn new(web3: &web3::Web3<T>, address: H160) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contract/api3_pool.abi.json"),
        )
        .expect("fail contract::from_json(api3_pool.abi.json)");
        Pool { contract: contract }
    }

    pub async fn read(&self) -> Option<Api3PoolInfo> {
        let min_apr: U256 = match self
            .contract
            .query("minApr", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("minApr {}", e);
                return None;
            }
        };
        let max_apr: U256 = match self
            .contract
            .query("maxApr", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("maxApr {}", e);
                return None;
            }
        };
        let genesis_apr: f64 = nice::dec((min_apr + max_apr) / U256::from(2), 18 - 4) / 10e3;
        let epoch_length: U256 = match self
            .contract
            .query("EPOCH_LENGTH", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("EPOCH_LENGTH {}", e);
                return None;
            }
        };
        let rewards_coeff: f64 = if epoch_length.as_u64() == 3600 {
            1.0
        } else {
            let week: u64 = epoch_length.as_u64() / 3600 / 24;
            52.0 * (week as f64) / 365.0
        };
        let reward_vesting_period: U256 = match self
            .contract
            .query("REWARD_VESTING_PERIOD", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("REWARD_VESTING_PERIOD {}", e);
                return None;
            }
        };
        let unstake_wait_period: U256 = match self
            .contract
            .query("unstakeWaitPeriod", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("unstakeWaitPeriod {}", e);
                return None;
            }
        };
        let stake_target: U256 = match self
            .contract
            .query("stakeTarget", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("stakeTarget {}", e);
                return None;
            }
        };
        Some(Api3PoolInfo {
            genesis_apr,
            min_apr: nice::dec(min_apr, 18 - 4) / 10e3,
            max_apr: nice::dec(max_apr, 18 - 4) / 10e3,
            rewards_coeff,
            epoch_length: epoch_length.as_u64(),
            reward_vesting_period: reward_vesting_period.as_u64(),
            unstake_wait_period: unstake_wait_period.as_u64(),
            stake_target,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Supply<T>
where
    T: web3::Transport,
{
    contract: Contract<T>,
    addr_convenience: H160,
    addr_primary: H160,
    addr_secondary: H160,
}

impl<T: web3::Transport> Supply<T> {
    pub fn new(
        web3: &web3::Web3<T>,
        address: H160,
        addr_convenience: H160,
        addr_primary: H160,
        addr_secondary: H160,
    ) -> Self {
        tracing::info!("reading supply {:?}", address);
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contract/api3_supply.abi.json"),
        )
        .expect("fail contract::from_json(api3_supply.abi.json)");
        Supply {
            contract,
            addr_convenience,
            addr_primary,
            addr_secondary,
        }
    }

    pub async fn read(&self) -> Option<Api3Circulation> {
        let opt = Options::with(|opt| {
            // opt.value = Some(5.into());
            opt.gas = Some(10_000_000.into());
        });
        let locked_by_governance: U256 = match self
            .contract
            .query("getLockedByGovernance", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getLockedByGovernance {}", e);
                return None;
            }
        };

        let locked_rewards: U256 = match self
            .contract
            .query("getLockedRewards", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getLockedRewards {}", e);
                return None;
            }
        };
        let locked_vestings: U256 = match self
            .contract
            .query("getLockedVestings", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getLockedVestings {}", e);
                return None;
            }
        };
        let time_locked: U256 = match self
            .contract
            .query("getTimelocked", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getTimelocked {}", e);
                return None;
            }
        };
        let total_locked: U256 = match self
            .contract
            .query("getTotalLocked", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getTotalLocked {}", e);
                return None;
            }
        };
        let circulating_supply: U256 = match self
            .contract
            .query("getCirculatingSupply", (), None, opt.clone(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("getCirculatingSupply {}", e);
                return None;
            }
        };

        let addr_pool: H160 = match self
            .contract
            .query("API3_POOL", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("API3_POOL {}", e);
                return None;
            }
        };
        let addr_token: H160 = match self
            .contract
            .query("API3_TOKEN", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("API3_TOKEN {}", e);
                return None;
            }
        };
        let addr_time_lock: H160 = match self
            .contract
            .query("TIMELOCK_MANAGER", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("TIMELOCK_MANAGER {}", e);
                return None;
            }
        };
        let addr_primary_treasury: H160 = match self
            .contract
            .query("PRIMARY_TREASURY", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("PRIMARY_TREASURY {}", e);
                return None;
            }
        };
        let addr_secondary_treasury: H160 = match self
            .contract
            .query("SECONDARY_TREASURY", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("SECONDARY_TREASURY {}", e);
                return None;
            }
        };
        let addr_v1_treasury: H160 = match self
            .contract
            .query("V1_TREASURY", (), None, Options::default(), None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                warn!("V1_TREASURY {}", e);
                return None;
            }
        };

        Some(Api3Circulation {
            circulating_supply,
            locked_by_governance,
            locked_rewards,
            locked_vestings,
            time_locked,
            total_locked,
            addr_pool,
            addr_token,
            addr_time_lock,
            addr_primary_treasury,
            addr_secondary_treasury,
            addr_v1_treasury,
            addr_convenience: self.addr_convenience.clone(),
            addr_primary_contract: self.addr_primary.clone(),
            addr_secondary_contract: self.addr_secondary.clone(),
        })
    }
}
