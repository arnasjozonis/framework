use crate::beacon_node::BeaconNode;
use bls::{SecretKey, Signature};
use ssz_types::BitList;
use tree_hash::TreeHash;
use types::beacon_state::BeaconState;
use types::config::*;
use types::primitives::{Epoch, H256};
use types::types::{
    Attestation, AttestationData, AttestationDataAndCustodyBit, AttestationDuty, Checkpoint,
};

pub struct AttestationProducer<C: Config, BN: BeaconNode<C>> {
    pub config: C,
    pub beacon_node: BN,
}

impl<C: Config, BN: BeaconNode<C>> AttestationProducer<C, BN> {
    pub fn construct_attestation_data(&mut self, head_state: &BeaconState<C>) -> AttestationData {
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
            index: 0, // TODO: set to correct one from service
            slot: head_state.slot,
            beacon_block_root: self.beacon_node.get_block_root(head_state, epoch).unwrap(),
            source: head_state.current_justified_checkpoint.clone(),
            target,
        };

        attestation_data
    }

    pub fn get_signed_attestation_data(
        &mut self,
        state: &BeaconState<C>,
        attestation_data: &AttestationData,
        privkey: &SecretKey,
    ) -> Signature {
        let DOMAIN_BEACON_ATTESTER = 1;

        let domain = self.beacon_node.get_domain(
            state,
            DOMAIN_BEACON_ATTESTER,
            Some(attestation_data.target.epoch),
        );
        Signature::new(&attestation_data.tree_hash_root()[..], domain, privkey)
    }

    pub fn construct_attestation(
        &mut self,
        head_state: &BeaconState<C>,
        attestation_data: AttestationData,
        attestation_duty: AttestationDuty,
    ) -> Option<Attestation<C>> {
        let mut aggregation_bits = BitList::with_capacity(attestation_duty.committee_len).ok()?;
        aggregation_bits
            .set(attestation_duty.committee_index, true)
            .ok()?;

        let privkey = SecretKey::random();
        let signed_attestation_data =
            self.get_signed_attestation_data(head_state, &attestation_data, &privkey);

        Some(Attestation {
            aggregation_bits,
            data: attestation_data.clone(),
            custody_bits: BitList::with_capacity(attestation_duty.committee_len).ok()?,
            signature: signed_attestation_data,
        })
    }
}
