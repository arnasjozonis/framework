use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, ValidatorIndex, H256};

#[derive(PartialEq, Debug)]
pub enum Error {
    SlotOutOfRange,
    IndexOutOfRange,
}

pub trait BeaconNode<C: Config> {
    fn get_current_epoch(&self, state: &BeaconState<C>) -> Epoch;
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    fn get_committee_count_at_slot(&self, state: &BeaconState<C>, slot: Slot) -> u64;
    fn get_beacon_committee(
        &self,
        state: &BeaconState<C>,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex>;
    fn get_beacon_proposer_index(&self, state: &BeaconState<C>) -> ValidatorIndex;
    fn get_block_root(&self, state: &BeaconState<C>, epoch: Epoch) -> Result<H256, Error>;
    fn get_block_root_at_slot(&self, state: &BeaconState<C>, slot: Slot) -> Result<H256, Error>;
    fn get_domain(
        &self,
        state: &BeaconState<C>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain;
}

pub struct BasicBeaconNode<C: Config> {
    pub Cfg: C,
}

impl<C: Config> BeaconNode<C> for BasicBeaconNode<C> {
    fn get_current_epoch(&self, state: &BeaconState<C>) -> Epoch {
        let res: Epoch = 0;
        res
    }
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot {
        let res: Slot = 0;
        res
    }
    fn get_committee_count_at_slot(&self, state: &BeaconState<C>, slot: Slot) -> u64 {
        let res: u64 = 4;
        res
    }
    fn get_beacon_committee(
        &self,
        state: &BeaconState<C>,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex> {
        let mut res: Vec<ValidatorIndex> = Vec::new();
        res.push(0);
        res.push(1);
        res.push(2);
        res.push(3);
        res
    }
    fn get_beacon_proposer_index(&self, state: &BeaconState<C>) -> ValidatorIndex {
        let res: ValidatorIndex = 3;
        res
    }
    fn get_block_root(&self, state: &BeaconState<C>, epoch: Epoch) -> Result<H256, Error> {
        Err(Error::IndexOutOfRange)
    }
    fn get_block_root_at_slot(&self, state: &BeaconState<C>, slot: Slot) -> Result<H256, Error> {
        Ok(H256::from([0; 32]))
    }
    fn get_domain(
        &self,
        state: &BeaconState<C>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain {
        0
    }
}
