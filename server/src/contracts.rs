use client::state::{Api3Circulation, Api3PoolInfo};
use hex_literal::hex;
use web3::contract::{Contract, Options};
use web3::types::{H160, U256};

#[derive(Debug)]
pub struct Pool<T>
where
    T: web3::Transport,
{
    web3: web3::Web3<T>,
    contract: Contract<T>,
}

impl<T: web3::Transport> Pool<T> {
    pub fn new(web3: web3::Web3<T>, address: H160) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contract/api3_pool.abi.json"),
        )
        .expect("fail contract::from_json(api3_pool.abi.json)");
        Pool {
            web3: web3,
            contract: contract,
        }
    }

    pub async fn read(&self) -> Option<Api3PoolInfo> {
        let min_apr: f64 = 0.0;
        let max_apr: f64 = 100.0;
        let genesis_apr: f64 = (min_apr + max_apr) * 0.5;
        let rewards_coeff: f64 = 1.0;
        let epoch_length: u64 = 1;
        let reward_vesting_period: u64 = 0;
        let stake_target: U256 = U256::from(0);
        let unstake_wait_period: u64 = 0;
        Some(Api3PoolInfo {
            genesis_apr,
            min_apr,
            max_apr,
            rewards_coeff,
            epoch_length,
            reward_vesting_period,
            stake_target,
            unstake_wait_period,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Supply<T>
where
    T: web3::Transport,
{
    web3: web3::Web3<T>,
    contract: Contract<T>,
}

impl<T: web3::Transport> Supply<T> {
    pub fn new(web3: web3::Web3<T>, address: H160) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("./contract/api3_supply.abi.json"),
        )
        .expect("fail contract::from_json(api3_supply.abi.json)");
        Supply {
            web3: web3,
            contract: contract,
        }
    }

    pub async fn read(&self) -> Option<Api3Circulation> {
        let circulating_supply: U256 = U256::from(0);
        let locked_by_governance: U256 = U256::from(0);
        let locked_rewards: U256 = U256::from(0);
        let locked_vestings: U256 = U256::from(0);
        let time_locked: U256 = U256::from(0);
        let total_locked: U256 = U256::from(0);

        let addr_pool: H160 = hex!("0000000000000000000000000000000000000000").into();
        let addr_token: H160 = hex!("0000000000000000000000000000000000000000").into();
        let addr_time_lock: H160 = hex!("0000000000000000000000000000000000000000").into();
        let addr_primary_treasury: H160 = hex!("0000000000000000000000000000000000000000").into();
        let addr_secondary_treasury: H160 = hex!("0000000000000000000000000000000000000000").into();
        let addr_v1_treasury: H160 = hex!("0000000000000000000000000000000000000000").into();

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
        })
    }
}

// let abi = web3::ethabi::Contract::load(std::io::Cursor::new(include_bytes!("./contract/api3_pool.abi.jsons")))?;
// let contract = Contract::new(web3.eth(), addr, abi);
// let my_account: H160 = hex!("6518C695CdcbefA272a4E5EF73bD46E801983E19").into();
// let user_voting_power = contract
//     .query(
//         "userVotingPower",
//         (my_account,),
//         None,
//         Options::default(),
//         None,
//     )
//     .await?;
// println!(
//     "userVotingPower = {:?}",
//     nice::amount(user_voting_power, 18)
// );
// let user_shares = contract
//     .query("userShares", (my_account,), None, Options::default(), None)
//     .await?;
// println!("userShares = {:?}", nice::amount(user_shares, 18));
// let user_stake = contract
//     .query("userStake", (my_account,), None, Options::default(), None)
//     .await?;
// println!("userStake = {:?}", nice::amount(user_stake, 18));
