use types::config::Config;
use types::beacon_state::{ BeaconState };
use types::primitives::{ Epoch, ValidatorIndex, Slot };
use crate::beacon_node::BeaconNode;
use std::result;

pub enum WorkInfo {
    SignBlock,
    Attest,
    None
}

pub struct DutiesManager<C: Config, BN: BeaconNode<C>> {
    pub config: C,
    //pub validator: Validator,
    pub beacon_node: BN
}

impl<C: Config, BN: BeaconNode<C>> DutiesManager<C, BN> {
    pub fn get_duty(beacon_state: &BeaconState<C>, epoch: &Epoch, validator_index: &ValidatorIndex) -> Result<WorkInfo, i8> {
        // check if epoch passed to fn is valid
        let next_epoch = get_current_epoch(beacon_state) + 1;
        if next_epoch < *epoch { return Err(-1); };

        //
        let start_slot: Slot = compute_start_slot_at_epoch(epoch);
        for slot in start_slot..10 {
            let committee_count = get_committee_count_at_slot(beacon_state, slot);
            for index in 0..committee_count {
                let committee = get_beacon_committee(beacon_state, slot, index);

            }
        }
        
    
        Ok(WorkInfo::None)
    }
}