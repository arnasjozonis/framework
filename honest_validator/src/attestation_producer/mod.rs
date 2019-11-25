use crate::beacon_node::{BeaconNode, BeaconState, Attestation, AttestationData};
use bls::{SecretKey, Signature};
use ssz_types::BitList;
use types::config::*;
use types::types::{
    AttestationDuty, Checkpoint,
};

pub struct AttestationProducer<C: Config, BN: BeaconNode> {
    pub config: C,
    pub beacon_node: BN,
}

impl<C: Config, BN: BeaconNode> AttestationProducer<C, BN> {
    pub fn construct_attestation_data(&mut self, head_state: &BeaconState) -> AttestationData {
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

        // TODO: initialize with field values
        let attestation_data = AttestationData::default();
        attestation_data
    }

    pub fn get_signed_attestation_data(
        &mut self,
        state: &BeaconState,
        attestation_data: &AttestationData,
        privkey: &SecretKey,
    ) -> Signature {
        let DOMAIN_BEACON_ATTESTER = 1;

        let domain = self.beacon_node.get_domain(
            state,
            DOMAIN_BEACON_ATTESTER,
            Some(attestation_data.target_epoch),
        );
        Signature::new(&attestation_data.target_root.as_bytes(), domain, privkey)
    }

    pub fn construct_attestation(
        &mut self,
        head_state: &BeaconState,
        attestation_data: AttestationData,
        attestation_duty: AttestationDuty,
    ) -> Option<Attestation> {
        // let mut aggregation_bits:  = BitList::with_capacity(attestation_duty.committee_len).ok()?;
        // aggregation_bits
        //     .set(attestation_duty.committee_index, true)
        //     .ok()?;

        // let privkey = SecretKey::random();
        // let signed_attestation_data =
        //     self.get_signed_attestation_data(head_state, &attestation_data, &privkey);

        // TODO: return attestation with fields containing relevant values
        Some(Attestation::default())
    }
}
