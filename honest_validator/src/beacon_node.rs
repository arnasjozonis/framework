use types::primitives::{Epoch, ValidatorIndex, Slot, CommitteeIndex};
use types::beacon_state::BeaconState;
use types::config::Config;

pub trait BeaconNode<C: Config> {
    fn get_current_epoch(&self, state: BeaconState<C>) -> Epoch;
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    fn get_committee_count_at_slot(&self, state: BeaconState<C>, slot: Slot) -> u64;
    fn get_beacon_committee(&self, state: BeaconState<C>, slot: Slot, index: CommitteeIndex) -> Vec<ValidatorIndex>;
}

pub struct BasicBeaconNode<C: Config>{
    cfg: C
}
impl<C: Config> BasicBeaconNode<C> {
    pub fn new(cfg: C) -> BasicBeaconNode<C> {
        BasicBeaconNode { cfg }
    }
}

impl<C: Config> BeaconNode<C> for BasicBeaconNode<C> {
     fn get_current_epoch(&self, state: BeaconState<C>) -> Epoch { let res: Epoch = 0; res}
     fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot { let res: Slot = 0; res}
     fn get_committee_count_at_slot(&self, state: BeaconState<C>, slot: Slot) -> u64 { let res: u64 = 0; res}
     fn get_beacon_committee(&self, state: BeaconState<C>, slot: Slot, index: CommitteeIndex) -> Vec<ValidatorIndex> {
        let res: Vec<ValidatorIndex> = Vec::new();
        res
    }
}