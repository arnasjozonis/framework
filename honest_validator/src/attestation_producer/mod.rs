use crate::beacon_node::{BasicBeaconNode, BeaconNode};
use bls::{SecretKey, Signature};
use ssz_types::BitList;
use std::convert::TryInto;
use tree_hash::TreeHash;
use types::beacon_state::BeaconState;
use types::config::*;
use types::primitives::{CommitteeIndex, Slot, ValidatorIndex};
use types::types::{Attestation, AttestationData, Checkpoint};

const MAX_VALIDATORS_PER_COMMITTEE: usize = 4;

pub struct AttestationProducer<C: Config> {
    pub config: C,
    pub beacon_node: BasicBeaconNode,
}

impl<C: Config> AttestationProducer<C> {
    fn construct_attestation_data(
        &self,
        head_state: &BeaconState<MinimalConfig>,
        assigned_slot: Slot,
        committee_index: CommitteeIndex,
    ) -> AttestationData {
        let epoch = self.beacon_node.get_current_epoch(head_state);

        let start_slot = self.beacon_node.compute_start_slot_at_epoch(epoch);

        let head_block_root = head_state.latest_block_header.state_root;

        let epoch_boundary_block_root = if start_slot == head_state.slot {
            head_block_root
        } else {
            self.beacon_node
                .get_block_root_at_slot(head_state, start_slot)
                .unwrap()
        };

        let target = Checkpoint {
            epoch: epoch,
            root: epoch_boundary_block_root,
        };

        let attestation_data = AttestationData {
            index: committee_index,
            slot: assigned_slot,
            beacon_block_root: head_block_root,
            source: head_state.current_justified_checkpoint.clone(),
            target,
        };

        attestation_data
    }

    fn get_signed_attestation_data(
        &self,
        state: &BeaconState<MinimalConfig>,
        attestation_data: &AttestationData,
        privkey: SecretKey,
    ) -> Signature {
        let domain = self.beacon_node.get_domain(
            state,
            MinimalConfig::domain_attestation(),
            Some(attestation_data.target.epoch),
        );
        Signature::new(&attestation_data.tree_hash_root()[..], domain, &privkey)
    }

    fn construct_attestation(
        &self,
        head_state: &BeaconState<MinimalConfig>,
        attestation_data: AttestationData,
        validator_committee_index: ValidatorIndex,
        privkey: SecretKey,
    ) -> Option<Attestation<MinimalConfig>> {
        let mut aggregation_bits = BitList::with_capacity(MAX_VALIDATORS_PER_COMMITTEE)
            .ok()
            .unwrap();
        aggregation_bits
            .set(validator_committee_index.try_into().unwrap(), true)
            .unwrap();

        let signed_attestation_data =
            self.get_signed_attestation_data(head_state, &attestation_data, privkey);

        Some(Attestation {
            aggregation_bits,
            data: attestation_data.clone(),
            signature: signed_attestation_data,
        })
    }

    pub fn get_attestation(
        &self,
        beacon_state: &BeaconState<MinimalConfig>,
        commitee_index: CommitteeIndex,
        validator_commitee_index: ValidatorIndex,
        privkey: SecretKey,
    ) -> Option<Attestation<MinimalConfig>> {
        println!(
            "Validator at committe {} (position {}) starts attestation",
            commitee_index, validator_commitee_index
        );

        let attestation_data =
            self.construct_attestation_data(&beacon_state, beacon_state.slot, commitee_index);

        self.construct_attestation(
            &beacon_state,
            attestation_data,
            validator_commitee_index,
            privkey,
        )
    }
}
