use types::config::{ QuickConfig };
use types::beacon_state::{ BeaconState };
use types::primitives::{ Epoch, ValidatorIndex, Slot };
use crate::beacon_node::{ BasicBeaconNode, BeaconNode };

pub enum WorkInfo {
    SignBlock,
    Attest,
    None
}

pub trait Worker {
    type SuccessType;
    type ErrorType;
    fn do_work(&self, state: &BeaconState<QuickConfig>) -> Result<Self::SuccessType, Self::ErrorType>;
}

pub struct DutiesManager
{
    beacon_node: BasicBeaconNode<QuickConfig>
}

impl DutiesManager{

    pub fn new(beacon_node: BasicBeaconNode<QuickConfig>) -> DutiesManager {
        DutiesManager { beacon_node }
    }
    
    pub fn get_duty(
        &self,
        beacon_state: &BeaconState<QuickConfig>,
        epoch: Epoch,
        validator_index: ValidatorIndex
    ) -> Result<WorkInfo, i8> {

        // check if epoch passed to fn is valid
        let next_epoch = self.beacon_node.get_current_epoch(beacon_state) + 1;
        if next_epoch < epoch { return Err(-1); };

        let start_slot: Slot = self.beacon_node.compute_start_slot_at_epoch(epoch);
        //TODO: take value from config
        let end_slot: Slot = 8;

        for slot in start_slot..end_slot {
            let committee_count = self.beacon_node.get_committee_count_at_slot(beacon_state, slot);
            for index in 0..committee_count {
                let committee = self.beacon_node.get_beacon_committee(beacon_state, slot, index);
                let assignment = committee.iter().find(|&&idx| {
                    idx == validator_index
                });
                return match assignment {
                    Some(val) => Ok(WorkInfo::Attest),
                    None => Ok(WorkInfo::None)
                }
            }
        }
        
        Ok(WorkInfo::None)
    }
}

pub struct TestWorker {
    
}

impl Worker for TestWorker {
    type SuccessType = &'static str;
    type ErrorType = u8;
    fn do_work(&self, state: &BeaconState<QuickConfig>) -> Result<&'static str, u8> {
        let slot = state.slot;
        if slot == Slot::default() {
            Ok("Default values work")
        } else {
            Err(0)
        }
    }
}