//temporary Lighthouse SSZ and hashing implementation
use bls::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use serde_bytes;
use ssz_derive::{Decode, Encode};
use ssz_types::{BitList, FixedVector, VariableList};
use tree_hash::TreeHash;
use tree_hash_derive::{SignedRoot, TreeHash};
use typenum::{Sum, U1};

use crate::config::*;
use crate::consts;
use crate::primitives::*;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot)]
pub struct Attestation<C: Config> {
    pub aggregation_bits: BitList<C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub signature: Signature,
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Hash,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    TreeHash,
    SignedRoot,
    Default,
)]
pub struct AttestationData {
    pub slot: Slot,
    pub index: CommitteeIndex,
    pub beacon_block_root: H256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Debug,
    Default,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    TreeHash,
    SignedRoot,
)]
pub struct AttestationDuty {
    pub slot: Slot,
    pub shard: Shard,
    pub committee_index: usize,
    pub committee_len: usize,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct AttesterSlashing<C: Config> {
    pub attestation_1: IndexedAttestation<C>,
    pub attestation_2: IndexedAttestation<C>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot)]
pub struct BeaconBlock<C: Config> {
    pub slot: Slot,
    pub parent_root: H256,
    pub state_root: H256,
    pub body: BeaconBlockBody<C>,
    #[signed_root(skip_hashing)]
    pub signature: Signature,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot)]
pub struct BeaconBlockBody<C: Config> {
    pub randao_reveal: Signature,
    pub eth1_data: Eth1Data,
    pub graffiti: [u8; 32],
    pub proposer_slashings: VariableList<ProposerSlashing, C::MaxProposerSlashings>,
    pub attester_slashings: VariableList<AttesterSlashing<C>, C::MaxAttesterSlashings>,
    pub attestations: VariableList<Attestation<C>, C::MaxAttestations>,
    pub deposits: VariableList<Deposit, C::MaxDeposits>,
    pub voluntary_exits: VariableList<VoluntaryExit, C::MaxVoluntaryExits>,
    pub transfers: VariableList<Transfer, C::MaxTransfers>,
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    TreeHash,
    SignedRoot,
    Default,
)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub parent_root: H256,
    pub state_root: H256,
    pub body_root: H256,
    pub signature: Signature,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Default, Hash, Deserialize, Serialize, Encode, Decode, TreeHash,
)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: H256,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Default, Hash, Deserialize, Serialize, Encode, Decode, TreeHash,
)]
pub struct Crosslink {
    pub shard: u64,
    pub parent_root: H256,
    pub start_epoch: Epoch,
    pub end_epoch: Epoch,
    pub data_root: H256,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct Deposit {
    pub proof: FixedVector<H256, Sum<consts::DepositContractTreeDepth, U1>>,
    pub data: DepositData,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot,
)]
pub struct DepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: H256,
    pub amount: u64,
    #[signed_root(skip_hashing)]
    pub signature: Signature,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct Eth1Data {
    pub deposit_root: H256,
    pub deposit_count: u64,
    pub block_hash: H256,
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    TreeHash,
    SignedRoot,
    Default,
)]
pub struct Fork {
    #[serde(with = "serde_bytes")]
    pub previous_version: Version,
    #[serde(with = "serde_bytes")]
    pub current_version: Version,
    pub epoch: Epoch,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct HistoricalBatch<C: Config> {
    pub block_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    pub state_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot)]
pub struct IndexedAttestation<C: Config> {
    pub custody_bit_0_indices: VariableList<u64, C::MaxValidatorsPerCommittee>,
    pub custody_bit_1_indices: VariableList<u64, C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    #[signed_root(skip_hashing)]
    pub signature: Signature,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct PendingAttestation<C: Config> {
    pub aggregation_bits: BitList<C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub inclusion_delay: u64,
    pub proposer_index: u64,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash)]
pub struct ProposerSlashing {
    pub proposer_index: u64,
    pub header_1: BeaconBlockHeader,
    pub header_2: BeaconBlockHeader,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot,
)]
pub struct Transfer {
    pub sender: u64,
    pub recipient: u64,
    pub amount: u64,
    pub fee: u64,
    pub slot: Slot,
    pub pubkey: PublicKey,
    #[signed_root(skip_hashing)]
    pub signature: Signature,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, Default)]
pub struct Validator {
    pub pubkey: PublicKey,
    pub withdrawal_credentials: H256,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot,
)]
pub struct VoluntaryExit {
    pub epoch: Epoch,
    pub validator_index: u64,
    #[signed_root(skip_hashing)]
    pub signature: Signature,
}

#[derive(
    Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Encode, Decode, TreeHash, SignedRoot,
)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: Signature,
}
