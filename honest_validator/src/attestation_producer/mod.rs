use bls::Signature;
use ssz_types::BitList;
use tree_hash::TreeHash;
use types::config::*;
use types::types::{Attestation, AttestationData, AttestationDataAndCustodyBit, AttestationDuty};

pub struct AttestationProducer<C: Config> {
    pub config: C,
}

impl<C: Config> AttestationProducer<C> {
    /*pub fn construct_attestation(attestation_duty: AttestationDuty) {
        // get epoch from attestation_duty.slot

        let attestation_data = AttestationData {
            beacon_block_root: signing_root
        }
    }*/

    pub fn sign_attestation_data(
        attestation_data: AttestationData,
        attestation_duty: AttestationDuty,
    ) -> Option<Attestation<C>> {
        let mut aggregation_bits = BitList::with_capacity(attestation_duty.committee_len).ok()?;
        aggregation_bits
            .set(attestation_duty.committee_index, true)
            .ok()?;

        let _data_with_bit = AttestationDataAndCustodyBit {
            data: attestation_data.clone(),
            custody_bit: false,
        }
        .tree_hash_root();

        // TODO sign data_with_bit
        let aggregate_signature = Signature::empty_signature();

        Some(Attestation {
            aggregation_bits,
            data: attestation_data,
            custody_bits: BitList::with_capacity(attestation_duty.committee_len).ok()?,
            signature: aggregate_signature,
        })
    }
}
