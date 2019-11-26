use crate::beacon_node::{BasicBeaconNode, BeaconNode};
use typenum::*;
use types::beacon_state::BeaconState;
use types::config::{Config, MinimalConfig};
use types::primitives::{Epoch, Slot, ValidatorIndex};

pub enum WorkInfo {
    SignBlock,
    Attest,
    None,
}

pub trait Worker {
    type SuccessType;
    type ErrorType;
    fn do_work(
        &self,
        state: &BeaconState<MinimalConfig>,
    ) -> Result<Self::SuccessType, Self::ErrorType>;
}

pub struct DutiesManager {}

impl DutiesManager {
    pub fn get_duty(
        beacon_state: &BeaconState<MinimalConfig>,
        epoch: Epoch,
        validator_index: ValidatorIndex,
        beacon_node: &BasicBeaconNode,
    ) -> Result<WorkInfo, String> {
        let next_epoch = beacon_node.get_current_epoch(beacon_state) + 1;
        if next_epoch < epoch {
            return Err(String::from(
                "Epoch to request duties is too far in the future",
            ));
        };

        let start_slot: Slot = beacon_node.compute_start_slot_at_epoch(epoch);
        let end_slot = <MinimalConfig as Config>::SlotsPerEpoch::to_u64() + &start_slot;
        for slot in start_slot..end_slot {
            let committee_count = beacon_node.get_committee_count_at_slot(beacon_state, slot);
            for index in 0..committee_count {
                let committee = beacon_node.get_beacon_committee(beacon_state, slot, index);
                let assignment = committee.iter().find(|&&idx| idx == validator_index);
                return match assignment {
                    Some(val) => Ok(WorkInfo::Attest),
                    None => Ok(WorkInfo::None),
                };
            }
        }
        Ok(WorkInfo::None)
    }
}

pub struct TestWorker {}

impl Worker for TestWorker {
    type SuccessType = &'static str;
    type ErrorType = u8;
    fn do_work(&self, state: &BeaconState<MinimalConfig>) -> Result<&'static str, u8> {
        let slot = state.slot;
        if slot == Slot::default() {
            Ok("Default values work")
        } else {
            Err(0)
        }
    }
}
