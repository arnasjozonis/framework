use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, ValidatorIndex, H256};
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug)]
pub enum Error {
    SlotOutOfRange,
    IndexOutOfRange,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub root: String,
    pub beacon_state: BeaconState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeaconState {
    pub genesis_time: u64,
    pub slot: u64,
    pub fork: Fork,
    pub latest_block_header: LatestBlockHeader,
    pub block_roots: Vec<String>,
    pub state_roots: Vec<String>,
    pub historical_roots: Vec<String>,
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: Vec<Eth1DataVote>,
    pub eth1_deposit_index: u64,
    pub validators: Vec<Validator>,
    pub balances: Vec<u64>,
    pub start_shard: u64,
    pub randao_mixes: Vec<String>,
    pub active_index_roots: Vec<String>,
    pub compact_committees_roots: Vec<String>,
    pub slashings: Vec<u64>,
    pub previous_epoch_attestations: Vec<PreviousEpochAttestation>,
    pub current_epoch_attestations: Vec<CurrentEpochAttestation>,
    pub previous_crosslinks: Vec<PreviousCrosslink>,
    pub current_crosslinks: Vec<CurrentCrosslink>,
    pub justification_bits: Vec<bool>,
    pub previous_justified_checkpoint: PreviousJustifiedCheckpoint,
    pub current_justified_checkpoint: CurrentJustifiedCheckpoint,
    pub finalized_checkpoint: FinalizedCheckpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fork {
    pub previous_version: String,
    pub current_version: String,
    pub epoch: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestBlockHeader {
    pub slot: u64,
    pub parent_root: String,
    pub state_root: String,
    pub signature: String,
    pub body_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eth1Data {
    pub deposit_root: String,
    pub deposit_count: u64,
    pub block_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eth1DataVote {
    pub deposit_root: String,
    pub deposit_count: u64,
    pub block_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Validator {
    pub public_key: String,
    pub withdrawal_credentials: String,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligiblity_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: f64,
    pub withdrawable_epoch: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousEpochAttestation {
    pub aggregation_bits: Vec<bool>,
    pub data: Data,
    pub inclusion_delay: u64,
    pub proposer_index: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub beacon_block_root: String,
    pub source_epoch: u64,
    pub source_root: String,
    pub target_epoch: u64,
    pub target_root: String,
    pub crosslink: Crosslink,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crosslink {
    pub shard: u64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub parent_root: String,
    pub data_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentEpochAttestation {
    pub aggregation_bits: Vec<bool>,
    pub data: Data2,
    pub inclusion_delay: u64,
    pub proposer_index: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data2 {
    pub beacon_block_root: String,
    pub source_epoch: u64,
    pub source_root: String,
    pub target_epoch: u64,
    pub target_root: String,
    pub crosslink: Crosslink2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crosslink2 {
    pub shard: u64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub parent_root: String,
    pub data_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCrosslink {
    pub shard: u64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub parent_root: String,
    pub data_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentCrosslink {
    pub shard: u64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub parent_root: String,
    pub data_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousJustifiedCheckpoint {
    pub epoch: u64,
    pub root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentJustifiedCheckpoint {
    pub epoch: u64,
    pub root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalizedCheckpoint {
    pub epoch: u64,
    pub root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    pub aggregation_bitfield: String,
    pub custody_bitfield: String,
    pub signature: String,
    pub data: AttestationData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttestationData {
    pub beacon_block_root: String,
    pub source_epoch: u64,
    pub source_root: String,
    pub target_epoch: u64,
    pub target_root: String,
    pub crosslink: Crosslink,
}

pub trait BeaconNode {
    fn get_current_epoch(&self, state: &BeaconState) -> Epoch;
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    fn get_committee_count_at_slot(&self, state: &BeaconState, slot: Slot) -> u64;
    fn get_beacon_committee(
        &self,
        state: &BeaconState,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex>;
    fn get_beacon_proposer_index(&self, state: &BeaconState) -> ValidatorIndex;
    fn get_block_root(&self, state: &BeaconState, epoch: Epoch) -> Result<H256, Error>;
    fn get_block_root_at_slot(&self, state: &BeaconState, slot: Slot) -> Result<H256, Error>;
    fn get_domain(
        &self,
        state: &BeaconState,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain;
}

pub struct BasicBeaconNode {

}

impl BeaconNode for BasicBeaconNode {
    fn get_current_epoch(&self, state: &BeaconState) -> Epoch {
        let res: Epoch = 0;
        res
    }
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot {
        let res: Slot = 0;
        res
    }
    fn get_committee_count_at_slot(&self, state: &BeaconState, slot: Slot) -> u64 {
        let res: u64 = 4;
        res
    }
    fn get_beacon_committee(
        &self,
        state: &BeaconState,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex> {
        let mut res: Vec<ValidatorIndex> = Vec::new();
        res.push(0);
        res.push(1);
        res.push(2);
        res.push(3);
        res
    }
    fn get_beacon_proposer_index(&self, state: &BeaconState) -> ValidatorIndex {
        let res: ValidatorIndex = 3;
        res
    }
    fn get_block_root(&self, state: &BeaconState, epoch: Epoch) -> Result<H256, Error> {
        Err(Error::IndexOutOfRange)
    }
    fn get_block_root_at_slot(&self, state: &BeaconState, slot: Slot) -> Result<H256, Error> {
        Ok(H256::from([0; 32]))
    }
    fn get_domain(
        &self,
        state: &BeaconState,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain {
        0
    }
}
