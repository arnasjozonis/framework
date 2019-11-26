use crate::beacon_node::{BasicBeaconNode, BeaconNode};
use bls::{SecretKey, Signature};
use ssz_types::BitList;
use tree_hash::TreeHash;
use types::beacon_state::BeaconState;
use types::config::*;
use types::primitives::{CommitteeIndex, Slot, ValidatorIndex};
use types::types::{Attestation, AttestationData, Checkpoint};
use std::convert::TryInto;

pub struct AttestationProducer<C: Config> {
    pub config: C,
    pub beacon_node: BasicBeaconNode,
}

impl<C: Config> AttestationProducer<C> {
    pub fn construct_attestation_data(
        &mut self,
        head_state: &BeaconState<MinimalConfig>,
        assigned_slot: Slot,
        committee_index: CommitteeIndex,
    ) -> AttestationData {
        let epoch = self.beacon_node.get_current_epoch(head_state);

        let start_slot = self.beacon_node.compute_start_slot_at_epoch(epoch);
        let epoch_boundary_block_root = if start_slot == head_state.slot {
            self.beacon_node
                .get_block_root(head_state, head_state.slot)
                .unwrap()
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
            beacon_block_root: self.beacon_node.get_block_root(head_state, epoch).unwrap(),
            source: head_state.current_justified_checkpoint.clone(),
            target,
        };

        attestation_data
    }

    fn get_signed_attestation_data(
        &mut self,
        state: &BeaconState<MinimalConfig>,
        attestation_data: &AttestationData,
        privkey: &SecretKey,
    ) -> Signature {
        let domain = self.beacon_node.get_domain(
            state,
            MinimalConfig::domain_attestation(),
            Some(attestation_data.target.epoch),
        );
        Signature::new(&attestation_data.tree_hash_root()[..], domain, privkey)
    }

    pub fn construct_attestation(
        &mut self,
        head_state: &BeaconState<MinimalConfig>,
        attestation_data: AttestationData,
        assigned_slot: Slot,
        committee_index: CommitteeIndex,
        validator_index: ValidatorIndex,
    ) -> Option<Attestation<C>> {
        let committee_len = self
            .beacon_node
            .get_beacon_committee(head_state, assigned_slot, committee_index)
            .len();

        let mut aggregation_bits = BitList::with_capacity(committee_len).ok()?;
        aggregation_bits.set(validator_index.try_into().unwrap(), true).ok()?;

        let privkey = SecretKey::random();
        let signed_attestation_data =
            self.get_signed_attestation_data(head_state, &attestation_data, &privkey);

        Some(Attestation {
            aggregation_bits,
            data: attestation_data.clone(),
            signature: signed_attestation_data,
        })
    }
}
