#![allow(clippy::module_name_repetitions)]

use core::fmt::Debug;
use core::hash::Hash;

use serde::{Deserialize, Serialize};
use typenum::Unsigned;

use crate::primitives::{ValidatorIndex, DomainType};

pub trait Config
where
    Self: Clone + Copy + PartialEq + Eq + Hash + PartialOrd + Ord + Default + Debug,
{
    type EpochsPerSlashingsVector: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type EpochsPerHistoricalVector: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type HistoricalRootsLimit: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxAttesterSlashings: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxAttestations: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxAttestationsPerEpoch: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxDeposits: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxProposerSlashings: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxTransfers: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxValidatorsPerCommittee: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type MaxVoluntaryExits: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type ShardCount: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type SlotsPerEpoch: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type SlotsPerEth1VotingPeriod: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type SlotsPerHistoricalRoot: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;
    type ValidatorRegistryLimit: Unsigned
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + PartialOrd
        + Ord
        + Default
        + Debug;

    fn activation_exit_delay() -> u64 {
        4
    }
    fn base_reward_factor() -> u64 {
        64
    }
    fn bls_withdrawal_prefix_byte() -> u8 {
        0x00
    }
    fn churn_limit_quotient() -> u64 {
        0x0001_0000
    }
    fn domain_attestation() -> DomainType {
        1
    }
    fn domain_beacon_proposer() -> DomainType {
        0
    }
    fn domain_deposit() -> DomainType {
        3
    }
    fn domain_randao() -> DomainType {
        2
    }
    fn domain_transfer() -> DomainType {
        5
    }
    fn domain_voluntary_exit() -> DomainType {
        4
    }
    fn effective_balance_increment() -> u64 {
        1_000_000_000
    }
    fn ejection_balance() -> u64 {
        16_000_000_000
    }
    fn genesis_epoch() -> u64 {
        0
    }
    fn genesis_slot() -> u64 {
        0
    }
    fn inactivity_penalty_quotient() -> u64 {
        2_u64.pow(25)
    }
    fn max_effective_balance() -> u64 {
        32_000_000_000
    }
    fn max_epochs_per_crosslink() -> u64 {
        4
    }
    fn min_attestation_inclusion_delay() -> u64 {
        1
    }
    fn min_deposit_amount() -> u64 {
        1_000_000_000
    }
    fn min_epochs_to_inactivity_penalty() -> u64 {
        4
    }
    fn min_genesis_active_validator_count() -> u64 {
        64
    }
    // Bitcoin's 11th anniversary
    // (see <https://github.com/ethereum/eth2.0-specs/issues/1129#issue-448918350>).
    fn min_genesis_time() -> u64 {
        1_578_009_600
    }
    fn min_per_epoch_churn_limit() -> u64 {
        4
    }
    fn min_seed_lookahead() -> u64 {
        1
    }
    fn min_slashing_penalty_quotient() -> u64 {
        32
    }
    fn min_validator_withdrawability_delay() -> u64 {
        256
    }
    fn persistent_committee_period() -> u64 {
        2_u64.pow(11)
    }
    fn proposer_reward_quotient() -> u64 {
        8
    }
    fn shuffle_round_count() -> u64 {
        10
    }
    fn target_committee_size() -> u64 {
        4
    }
    fn whistleblower_reward_quotient() -> u64 {
        512
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug, Deserialize, Serialize,
)]
pub struct MainnetConfig {}

impl Config for MainnetConfig {
    type EpochsPerSlashingsVector = typenum::U64;
    type EpochsPerHistoricalVector = typenum::U64;
    type HistoricalRootsLimit = typenum::U16777216;
    type MaxAttesterSlashings = typenum::U1;
    type MaxAttestations = typenum::U128;
    type MaxAttestationsPerEpoch = typenum::U1024;
    type MaxDeposits = typenum::U16;
    type MaxProposerSlashings = typenum::U16;
    type MaxTransfers = typenum::U0;
    type MaxValidatorsPerCommittee = typenum::U4096;
    type MaxVoluntaryExits = typenum::U16;
    type ShardCount = typenum::U8;
    type SlotsPerEpoch = typenum::U8;
    type SlotsPerEth1VotingPeriod = typenum::U16;
    type SlotsPerHistoricalRoot = typenum::U64;
    type ValidatorRegistryLimit = typenum::U1099511627776;
}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug, Deserialize, Serialize,
)]
pub struct MinimalConfig;

impl Config for MinimalConfig {
    type EpochsPerSlashingsVector = typenum::U64;
    type EpochsPerHistoricalVector = typenum::U64;
    type HistoricalRootsLimit = typenum::U16777216;
    type MaxAttesterSlashings = typenum::U1;
    type MaxAttestations = typenum::U128;
    type MaxAttestationsPerEpoch = typenum::U1024;
    type MaxDeposits = typenum::U16;
    type MaxProposerSlashings = typenum::U16;
    type MaxTransfers = typenum::U0;
    type MaxValidatorsPerCommittee = typenum::U4096;
    type MaxVoluntaryExits = typenum::U16;
    type ShardCount = typenum::U8;
    type SlotsPerEpoch = typenum::U8;
    type SlotsPerEth1VotingPeriod = typenum::U16;
    type SlotsPerHistoricalRoot = typenum::U64;
    type ValidatorRegistryLimit = typenum::U1099511627776;
}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug, Deserialize, Serialize,
)]
pub struct QuickConfig;

impl Config for QuickConfig {
    type EpochsPerSlashingsVector = typenum::U64;
    type EpochsPerHistoricalVector = typenum::U64;
    type HistoricalRootsLimit = typenum::U16777216;
    type MaxAttesterSlashings = typenum::U1;
    type MaxAttestations = typenum::U128;
    type MaxAttestationsPerEpoch = typenum::U1024;
    type MaxDeposits = typenum::U16;
    type MaxProposerSlashings = typenum::U16;
    type MaxTransfers = typenum::U0;
    type MaxValidatorsPerCommittee = typenum::U4096;
    type MaxVoluntaryExits = typenum::U16;
    type ShardCount = typenum::U8;
    type SlotsPerEpoch = typenum::U1;
    type SlotsPerEth1VotingPeriod = typenum::U16;
    type SlotsPerHistoricalRoot = typenum::U64;
    type ValidatorRegistryLimit = typenum::U1099511627776;

    fn min_genesis_active_validator_count() -> ValidatorIndex {
        1
    }
    fn min_genesis_time() -> u64 {
        9_476_400
    }
}
