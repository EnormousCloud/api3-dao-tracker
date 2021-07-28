use crate::logreader::{EventParseError, LogReader};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use web3::types::{H160, U256};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VotingAgent {
    Primary,
    Secondary,
}

impl VotingAgent {
    pub fn as_int(&self) -> u64 {
        match self {
            Self::Primary => 0,
            Self::Secondary => 1,
        }
    }
}

pub fn voting_from_str(str: &str) -> (VotingAgent, u64) {
    let parts: Vec<&str> = str.split("-").collect();
    let first_char = parts[0][0..1].to_lowercase();
    let vote_id = parts[1].parse::<u64>().unwrap_or_default();
    (
        if first_char == "p" {
            VotingAgent::Primary
        } else {
            VotingAgent::Secondary
        },
        vote_id,
    )
}

pub fn voting_to_string(agent: &VotingAgent, vote_id: u64) -> String {
    let prefix = match agent {
        VotingAgent::Primary => "p",
        VotingAgent::Secondary => "s",
    };
    format!("{}-{}", prefix, vote_id)
}

pub fn voting_from_u64(src: u64) -> (VotingAgent, u64) {
    (
        if src % 2 == 0 {
            VotingAgent::Primary
        } else {
            VotingAgent::Secondary
        },
        src / 2,
    )
}

pub fn voting_to_u64(agent: &VotingAgent, vote_id: u64) -> u64 {
    vote_id * 2 + agent.as_int()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Api3 {
    // Pool contract events
    SetDaoApps {
        agent_app_primary: H160,
        agent_app_secondary: H160,
        voting_app_primary: H160,
        voting_app_secondary: H160,
    },
    Delegated {
        from: H160,
        to: H160,
        shares: U256,
        total_delegated_to: U256,
    },
    DelegatedV0 {
        from: H160,
        to: H160,
        shares: U256,
    },
    Undelegated {
        from: H160,
        to: H160,
        shares: U256,
        total_delegated_to: U256,
    },
    UndelegatedV0 {
        from: H160,
        to: H160,
        shares: U256,
    },
    UpdatedDelegation {
        user: H160,
        delegate: H160,
        delta: bool,
        shares: U256,
        total_delegated_to: U256,
    },
    Staked {
        user: H160,
        amount: U256,
        minted_shares: U256,
        user_unstaked: U256,
        user_shares: U256,
        total_shares: U256,
        total_stake: U256,
    },
    StakedV0 {
        user: H160,
        amount: U256,
        minted_shares: U256,
    },
    Unstaked {
        user: H160,
        amount: U256,
        user_unstaked: U256,
        total_shares: U256,
        total_stake: U256,
    },
    UnstakedV0 {
        user: H160,
        amount: U256,
    },
    ScheduledUnstake {
        user: H160,
        amount: U256,
        shares: U256,
        scheduled_for: U256,
        user_shares: U256,
    },
    ScheduledUnstakeV0 {
        user: H160,
        amount: U256,
        shares: U256,
        scheduled_for: U256,
    },
    Deposited {
        user: H160,
        amount: U256,
        user_unstaked: U256,
    },
    DepositedV0 {
        user: H160,
        amount: U256,
    },
    DepositedVesting {
        user: H160,
        amount: U256,
        start: U256,
        end: U256,
        user_unstaked: U256,
        user_vesting: U256,
    },
    DepositedByTimelockManager {
        user: H160,
        amount: U256,
        user_unstaked: U256,
    },
    VestedTimelock {
        user: H160,
        amount: U256,
        user_vesting: U256,
    },
    Withdrawn {
        user: H160,
        amount: U256,
        user_unstaked: U256,
    },
    WithdrawnV0 {
        user: H160,
        amount: U256,
    },
    WithdrawnToPool {
        recipient: H160,
        api3_pool_address: H160,
        beneficiary: H160,
    },
    UpdatedLastProposalTimestamp {
        user: H160,
        last_proposal_timestamp: U256,
        voting_app: H160,
    },
    SetStakeTarget {
        stake_target: U256,
    },
    MintedReward {
        epoch_index: U256,
        amount: U256,
        new_apr: U256,
        total_stake: U256,
    },
    MintedRewardV0 {
        epoch_index: U256,
        amount: U256,
        new_apr: U256,
    },
    // never happened yet
    PaidOutClaim {
        recipient: H160,
        amount: U256,
        total_stake: U256,
    },

    // Voting
    StartVote {
        agent: VotingAgent,
        vote_id: U256,
        creator: H160,
        metadata: String,
    },
    CastVote {
        agent: VotingAgent,
        vote_id: U256,
        voter: H160,
        supports: bool,
        stake: U256,
    },
    ExecuteVote {
        agent: VotingAgent,
        vote_id: U256,
    },

    // ChangeSupportRequired(U64), // 903b617f, never happened yet
    // ChangeMinQuorum(U64), // 3172f2e9, never happened yet

    // Convenience contract events
    SetErc20Addresses {
        addresses: Vec<H160>,
    },
    SetVestingAddresses {
        addresses: Vec<H160>,
    },
    OwnershipTransferred {
        from: H160,
        to: H160,
    },

    // ERC20 events:
    Transfer {
        from: H160,
        to: H160,
        amount: U256,
    },

    // unknown, but ignored, do not fail on this type
    Unclassified,
    // unknown and fail on that
    Unknown,
}

impl Api3 {
    pub fn is_broadcast(&self) -> bool {
        match &self {
            Self::MintedReward {
                epoch_index: _,
                amount: _,
                new_apr: _,
                total_stake: _,
            } => true,
            Self::MintedRewardV0 {
                epoch_index: _,
                amount: _,
                new_apr: _,
            } => true,
            _ => false,
        }
    }

    pub fn get_wallets(&self) -> Vec<H160> {
        let mut res: Vec<H160> = vec![];
        match self {
            Self::Delegated {
                from,
                to,
                shares: _,
                total_delegated_to: _,
            } => {
                res.push(from.clone());
                res.push(to.clone());
            }
            Self::DelegatedV0 {
                from,
                to,
                shares: _,
            } => {
                res.push(from.clone());
                res.push(to.clone());
            }
            Self::UpdatedDelegation {
                user,
                delegate,
                delta: _,
                shares: _,
                total_delegated_to: _,
            } => {
                res.push(user.clone());
                res.push(delegate.clone());
            }
            Self::Undelegated {
                from,
                to,
                shares: _,
                total_delegated_to: _,
            } => {
                res.push(from.clone());
                res.push(to.clone());
            }
            Self::UndelegatedV0 {
                from,
                to,
                shares: _,
            } => {
                res.push(from.clone());
                res.push(to.clone());
            }
            Self::Staked {
                user,
                amount: _,
                minted_shares: _,
                user_unstaked: _,
                user_shares: _,
                total_shares: _,
                total_stake: _,
            } => res.push(user.clone()),
            Self::StakedV0 {
                user,
                amount: _,
                minted_shares: _,
            } => res.push(user.clone()),
            Self::UnstakedV0 { user, amount: _ } => res.push(user.clone()),
            Self::ScheduledUnstake {
                user,
                amount: _,
                shares: _,
                scheduled_for: _,
                user_shares: _,
            } => res.push(user.clone()),
            Self::ScheduledUnstakeV0 {
                user,
                amount: _,
                shares: _,
                scheduled_for: _,
            } => res.push(user.clone()),
            Self::Deposited {
                user,
                amount: _,
                user_unstaked: _,
            } => res.push(user.clone()),
            Self::DepositedV0 { user, amount: _ } => res.push(user.clone()),
            Self::DepositedVesting {
                user,
                amount: _,
                start: _,
                end: _,
                user_unstaked: _,
                user_vesting: _,
            } => res.push(user.clone()),
            Self::DepositedByTimelockManager {
                user,
                amount: _,
                user_unstaked: _,
            } => res.push(user.clone()),
            Self::VestedTimelock {
                user,
                amount: _,
                user_vesting: _,
            } => res.push(user.clone()),
            Self::Withdrawn {
                user,
                amount: _,
                user_unstaked: _,
            } => res.push(user.clone()),
            Self::WithdrawnV0 { user, amount: _ } => res.push(user.clone()),
            Self::WithdrawnToPool {
                recipient,
                api3_pool_address: _,
                beneficiary,
            } => {
                res.push(recipient.clone());
                res.push(beneficiary.clone());
            }
            Self::PaidOutClaim {
                recipient,
                amount: _,
                total_stake: _,
            } => res.push(recipient.clone()),

            Self::StartVote {
                agent: _,
                vote_id: _,
                creator,
                metadata: _,
            } => res.push(creator.clone()),
            Self::CastVote {
                agent: _,
                vote_id: _,
                voter,
                supports: _,
                stake: _,
            } => res.push(voter.clone()),
            Self::SetVestingAddresses { addresses } => {
                for addr in addresses {
                    res.push(addr.clone());
                }
            }
            _ => {}
        };
        res
    }

    pub fn get_voting(&self) -> Option<u64> {
        match self {
            Self::StartVote {
                agent,
                vote_id,
                creator: _,
                metadata: _,
            } => Some(voting_to_u64(agent, vote_id.as_u64())),

            Self::CastVote {
                agent,
                vote_id,
                voter: _,
                supports: _,
                stake: _,
            } => Some(voting_to_u64(agent, vote_id.as_u64())),
            Self::ExecuteVote { agent, vote_id } => Some(voting_to_u64(agent, vote_id.as_u64())),
            _ => None,
        }
    }

    pub fn from_log(
        voting: Option<VotingAgent>,
        log: &web3::types::Log,
    ) -> Result<Self, EventParseError> {
        let t0 = log.topics[0];

        if t0 == hex!("24d7bda8602b916d64417f0dbfe2e2e88ec9b1157bd9f596dfdb91ba26624e04").into() {
            let mut r = LogReader::new(&log, 2, Some(2)).unwrap();
            return Ok(Self::Delegated {
                from: r.address(),
                to: r.address(),
                shares: r.value(),
                total_delegated_to: r.value(),
            });
        }
        if t0 == hex!("e5541a6b6103d4fa7e021ed54fad39c66f27a76bd13d374cf6240ae6bd0bb72b").into() {
            let mut r = LogReader::new(&log, 2, Some(1)).unwrap();
            return Ok(Self::DelegatedV0 {
                from: r.address(),
                to: r.address(),
                shares: r.value(),
            });
        }
        if t0 == hex!("3aace7340547de7b9156593a7652dc07ee900cea3fd8f82cb6c9d38b40829802").into() {
            let mut r = LogReader::new(&log, 2, Some(2)).unwrap();
            return Ok(Self::Undelegated {
                from: r.address(),
                to: r.address(),
                shares: r.value(),
                total_delegated_to: r.value(),
            });
        }
        if t0 == hex!("f310def5b4718cefe3603eb46259d8061fd58003695cf952de94c53e14dbb309").into() {
            let mut r = LogReader::new(&log, 2, Some(3)).unwrap();
            return Ok(Self::UpdatedDelegation {
                user: r.address(),
                delegate: r.address(),
                delta: r.bool(),
                shares: r.value(),
                total_delegated_to: r.value(),
            });
        }
        if t0 == hex!("4d10bd049775c77bd7f255195afba5088028ecb3c7c277d393ccff7934f2f92c").into() {
            let mut r = LogReader::new(&log, 2, Some(1)).unwrap();
            return Ok(Self::UndelegatedV0 {
                from: r.address(),
                to: r.address(),
                shares: r.value(),
            });
        }
        if t0 == hex!("251830cd12788c7474148132132ab205112e7b9bba739f0e69c8d4a6a54e2159").into() {
            let mut r = LogReader::new(&log, 1, Some(4)).unwrap();
            return Ok(Self::ScheduledUnstake {
                user: r.address(),
                amount: r.value(),
                shares: r.value(),
                scheduled_for: r.value(),
                user_shares: r.value(),
            });
        }
        if t0 == hex!("06fbd2297e6f6f7701a9cf99685a6af911cab275ec5c75ac7aaaf13b5cf3d61f").into() {
            let mut r = LogReader::new(&log, 1, Some(3)).unwrap();
            return Ok(Self::ScheduledUnstakeV0 {
                user: r.address(),
                amount: r.value(),
                shares: r.value(),
                scheduled_for: r.value(),
            });
        }
        if t0 == hex!("c16be9a586414a157dd46b4d023aa9997a025dd1cbbaa67ac0c1b8273a5eaf55").into() {
            let mut r = LogReader::new(&log, 1, Some(6)).unwrap();
            return Ok(Self::Staked {
                user: r.address(),
                amount: r.value(),
                minted_shares: r.value(),
                user_unstaked: r.value(),
                user_shares: r.value(),
                total_shares: r.value(),
                total_stake: r.value(),
            });
        }
        if t0 == hex!("1449c6dd7851abc30abf37f57715f492010519147cc2652fbc38202c18a6ee90").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::StakedV0 {
                user: r.address(),
                amount: r.value(),
                minted_shares: r.value(),
            });
        }
        if t0 == hex!("dcfd2b4017d03f7e541021db793b2f9b31e4acdee005f789e52853c390e3e962").into() {
            let mut r = LogReader::new(&log, 1, Some(4)).unwrap();
            return Ok(Self::Unstaked {
                user: r.address(),
                amount: r.value(),
                user_unstaked: r.value(),
                total_shares: r.value(),
                total_stake: r.value(),
            });
        }
        if t0 == hex!("0f5bb82176feb1b5e747e28471aa92156a04d9f3ab9f45f28e2d704232b93f75").into() {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::UnstakedV0 {
                user: r.address(),
                amount: r.value(),
            });
        }
        if t0 == hex!("92ccf450a286a957af52509bc1c9939d1a6a481783e142e41e2499f0bb66ebc6").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::Withdrawn {
                user: r.address(),
                amount: r.value(),
                user_unstaked: r.value(),
            });
        }
        if t0 == hex!("7084f5476618d8e60b11ef0d7d3f06914655adb8793e28ff7f018d4c76d505d5").into() {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::WithdrawnV0 {
                user: r.address(),
                amount: r.value(),
            });
        }
        if t0 == hex!("a2fd4f03989448c5a69bab0c0454f2baf5667413a4e4b87fd7379a8ab69fae3f").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::WithdrawnToPool {
                recipient: r.address(),
                api3_pool_address: r.address(),
                beneficiary: r.address(),
            });
        }
        if t0 == hex!("ceaef3a8d9336089c649bcf1ea9dd1ae52f5c42ea01f8707ecdd57ea773aa3ee").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::UpdatedLastProposalTimestamp {
                user: r.address(),
                last_proposal_timestamp: r.value(),
                voting_app: r.address(),
            });
        }
        if t0 == hex!("30df07121af80c9a50a8fcfddf8aa9f537a550edb930294c6370d4c05632ba15").into() {
            let mut r = LogReader::new(&log, 0, Some(1)).unwrap();
            return Ok(Self::SetStakeTarget {
                stake_target: r.value(),
            });
        }
        if t0 == hex!("71b1ce304e98c2a645f0c32f4c9e3ae4d5dbe6717a8c17ccefb0083635afdc15").into() {
            let mut r = LogReader::new(&log, 0, Some(4)).unwrap();
            return Ok(Self::SetDaoApps {
                agent_app_primary: r.address(),
                agent_app_secondary: r.address(),
                voting_app_primary: r.address(),
                voting_app_secondary: r.address(),
            });
        }
        if t0 == hex!("6e0fc10bac330e97bc2fd6c13cbb1c1189ddb48a8ce96395650ba8f2bd28f6fc").into() {
            let mut r = LogReader::new(&log, 1, Some(3)).unwrap();
            return Ok(Self::MintedReward {
                epoch_index: r.value(),
                amount: r.value(),
                new_apr: r.value(),
                total_stake: r.value(),
            });
        }
        if t0 == hex!("78fe37d5a5b277d7ec6fe20169a339795b44f3f903e0b793440f63fbccc7d7d9").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::MintedRewardV0 {
                epoch_index: r.value(),
                amount: r.value(),
                new_apr: r.value(),
            });
        }
        if t0 == hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").into() {
            let mut r = LogReader::new(&log, 2, Some(1)).unwrap();
            return Ok(Self::Transfer {
                from: r.address(),
                to: r.address(),
                amount: r.value(),
            });
        }
        if t0 == hex!("73a19dd210f1a7f902193214c0ee91dd35ee5b4d920cba8d519eca65a7b488ca").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::Deposited {
                user: r.address(),
                amount: r.value(),
                user_unstaked: r.value(),
            });
        }
        if t0 == hex!("2da466a7b24304f47e87fa2e1e5a81b9831ce54fec19055ce277ca2f39ba42c4").into() {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::DepositedV0 {
                user: r.address(),
                amount: r.value(),
            });
        }
        if t0 == hex!("14ab87851ecf43dc38c282e0307cd24257a3d01d0265ae2ba28764befac8c6cc").into() {
            let mut r = LogReader::new(&log, 1, Some(5)).unwrap();
            return Ok(Self::DepositedVesting {
                user: r.address(),
                amount: r.value(),
                start: r.value(),
                end: r.value(),
                user_unstaked: r.value(),
                user_vesting: r.value(),
            });
        }
        if t0 == hex!("d0d7fef3966369afd08c0683ee833a06f6b91787b85a26fa3ef3004ae37484c2").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::DepositedByTimelockManager {
                user: r.address(),
                amount: r.value(),
                user_unstaked: r.value(),
            });
        }
        if t0 == hex!("dd8c2c092b990b8e3ae25447982d1c2f7f08c6b9bf7303986a4279f946ebd2ea").into() {
            let mut r = LogReader::new(&log, 1, Some(2)).unwrap();
            return Ok(Self::VestedTimelock {
                user: r.address(),
                amount: r.value(),
                user_vesting: r.value(),
            });
        }

        if t0 == hex!("220c5b95388e82dd8e3a0abed6143750f9bfa4bf73bb6f742e10cf79e551b168").into() {
            let mut r = LogReader::new(&log, 0, None).unwrap();
            return Ok(Self::SetErc20Addresses {
                addresses: r.addresses(),
            });
        }
        if t0 == hex!("8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0").into() {
            let mut r = LogReader::new(&log, 2, Some(0)).unwrap();
            return Ok(Self::OwnershipTransferred {
                from: r.address(),
                to: r.address(),
            });
        }
        if t0 == hex!("4d72fe0577a3a3f7da968d7b892779dde102519c25527b29cf7054f245c791b9").into() {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            return Ok(Self::StartVote {
                agent: voting.unwrap(),
                vote_id: r.value(),
                creator: r.address(),
                metadata: r.text(),
            });
        }
        if t0 == hex!("b34ee265e3d4f5ec4e8b52d59b2a9be8fceca2f274ebc080d8fba797fea9391f").into() {
            let mut r = LogReader::new(&log, 2, Some(2)).unwrap();
            return Ok(Self::CastVote {
                agent: voting.unwrap(),
                vote_id: r.value(),
                voter: r.address(),
                supports: r.bool(),
                stake: r.value(),
            });
        }
        if t0 == hex!("bf8e2b108bb7c980e08903a8a46527699d5e84905a082d56dacb4150725c8cab").into() {
            let mut r = LogReader::new(&log, 1, Some(0)).unwrap();
            return Ok(Self::ExecuteVote {
                agent: voting.unwrap(),
                vote_id: r.value(),
            });
        }

        if t0 == hex!("9dcff9d94fbfdb4622d11edb383005f95e78efb446c72d92f8e615c6025c4703").into() {
            // happens right before agent app call,
            // parameters of agent call are changed in the reverse order
            // the last parameter is the agent address
            // found in: 0xc59489a810a16d84f59a04fb90817354d9afac3bd0a0b6787c8ccb4ff25ed119
            let _ = LogReader::new(&log, 3, Some(0)).unwrap();
            // return Ok(Self::???(r.address(), r.address.(), r.address()));
            return Ok(Self::Unclassified);
        }
        if t0 == hex!("c59489a810a16d84f59a04fb90817354d9afac3bd0a0b6787c8ccb4ff25ed119").into() {
            // that agent call
            let _ = LogReader::new(&log, 2, None).unwrap();
            // return Ok(Self::???(r.address(), r.address.(), r.data_ending_with_size()));
            return Ok(Self::Unclassified);
        }
        if t0 == hex!("5229a5dba83a54ae8cb5b51bdd6de9474cacbe9dd332f5185f3a4f4f2e3f4ad9").into() {
            // happens right after agent app call,
            // probably stores some return from the agent
            // found in: 0xc59489a810a16d84f59a04fb90817354d9afac3bd0a0b6787c8ccb4ff25ed119
            // found in: 0xa4e6695fd3d2185da9a004bbea2d82aedb548d081f68c116308b23339ccd36bf
            let _ = LogReader::new(&log, 1, None).unwrap();
            // return Ok(Self::???(r.address(),r.metadata()));
            return Ok(Self::Unclassified);
        }

        if t0 == hex!("2790b90165fd3973ad7edde4eca71b4f8808dd4857a2a3a3e8ae5642a5cb196e").into() {
            // most likely this is interaction with faucet
            let _ = LogReader::new(&log, 2, Some(1)).unwrap();
            // return Ok(Self::???(r.value(),r.address(),r.value()));
            return Ok(Self::Unclassified);
        }

        if t0 == hex!("c25cfed0b22da6a56f0e5ff784979a0b8623eddf2aee4acd33c2adefb09cbab6").into() {
            // most likely this is interaction with faucet
            let _ = LogReader::new(&log, 2, None).unwrap();
            // return Ok(Self::???(r.value(),r.address(),r.value()));
            return Ok(Self::Unclassified);
        }
        if t0 == hex!("20d5cc5c404f7bcf167ea08ea1136482041e05e5641946d3e3de6690a23fbe39").into() {
            let mut r = LogReader::new(&log, 0, None).unwrap();
            return Ok(Self::SetVestingAddresses {
                addresses: r.addresses(),
            });
        }
        Ok(Self::Unknown)
    }
}
