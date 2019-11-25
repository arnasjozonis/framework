use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, ValidatorIndex, H256};
use types::beacon_state::{BeaconState};
use types::config::{MinimalConfig, Config};
use serde::{Serialize, Deserialize};
use crate::rest_client::RestClient;

#[derive(PartialEq, Debug)]
pub enum Error {
    SlotOutOfRange,
    IndexOutOfRange,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BeaconStateResponse {
    pub root: String,
    pub beacon_state: BeaconState<MinimalConfig>,
}

pub trait BeaconNode {
    fn get_state(&self) -> &BeaconState<MinimalConfig>;
    fn get_current_epoch(&self, state: &BeaconState<MinimalConfig>) -> Epoch;
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    fn get_committee_count_at_slot(&self, state: &BeaconState<MinimalConfig>, slot: Slot) -> u64;
    fn get_beacon_committee(
        &self,
        state: &BeaconState<MinimalConfig>,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex>;
    fn get_beacon_proposer_index(&self, state: &BeaconState<MinimalConfig>) -> ValidatorIndex;
    fn get_block_root(&self, state: &BeaconState<MinimalConfig>, epoch: Epoch) -> Result<H256, Error>;
    fn get_block_root_at_slot(&self, state: &BeaconState<MinimalConfig>, slot: Slot) -> Result<H256, Error>;
    fn get_domain(
        &self,
        state: &BeaconState<MinimalConfig>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain;
}

pub struct BasicBeaconNode {
    pub bn: RestClient,
    last_known_state: BeaconState<MinimalConfig>,
}

impl BasicBeaconNode {
    pub fn new() -> BasicBeaconNode {
        let mut bn = RestClient::new(String::from("http://localhost:5052")).unwrap();
        let state: BeaconStateResponse = bn.get(&"/beacon/state").unwrap();
        BasicBeaconNode {
            bn,
            last_known_state: state.beacon_state
        }
    }

    pub fn update_state(&mut self) -> () {
        match self.bn.get(&"/beacon/state") {
            Some(state) => self.last_known_state = state,
            None => println!("failed update state")
        };
    }
}

impl BeaconNode for BasicBeaconNode {

    fn get_state(&self) -> &BeaconState<MinimalConfig> {
        &self.last_known_state
    }

    fn get_current_epoch(&self, state: &BeaconState<MinimalConfig>) -> Epoch {
        let res: Epoch = 0;
        res
    }
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot {
        let res: Slot = 0;
        res
    }
    fn get_committee_count_at_slot(&self, state: &BeaconState<MinimalConfig>, slot: Slot) -> u64 {
        let res: u64 = 4;
        res
    }
    fn get_beacon_committee(
        &self,
        state: &BeaconState<MinimalConfig>,
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
    fn get_beacon_proposer_index(&self, state: &BeaconState<MinimalConfig>) -> ValidatorIndex {
        let res: ValidatorIndex = 3;
        res
    }
    fn get_block_root(&self, state: &BeaconState<MinimalConfig>, epoch: Epoch) -> Result<H256, Error> {
        Err(Error::IndexOutOfRange)
    }
    fn get_block_root_at_slot(&self, state: &BeaconState<MinimalConfig>, slot: Slot) -> Result<H256, Error> {
        Ok(H256::from([0; 32]))
    }
    fn get_domain(
        &self,
        state: &BeaconState<MinimalConfig>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain {
        0
    }
}