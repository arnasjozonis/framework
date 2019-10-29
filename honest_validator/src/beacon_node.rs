use types::primitives:{Epoch, ValidatorIndex, Slot};
use beacon_state::BeaconState;

pub trait BeaconNode {
    fn get_current_epoch(state: BeaconState) -> Epoch;
    fn compute_start_slot_at_epoch(epoch: Epoch) -> Slot;
    fn get_committee_count_at_slot(state: BeaconState, slot: Slot) -> u64;
    fn get_beacon_committee(state: BeaconState, slot: Slot, index: CommitteeIndex) -> Vec[ValidatorIndex];
}
