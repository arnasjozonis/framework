use crate::rest_client::RestClient;
use serde::{Deserialize, Serialize};
use types::beacon_state::BeaconState;
use types::config::{MinimalConfig};
use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, H256};
use types::types::Attestation;
use std::rc::Rc;
use bls::PublicKeyBytes;

const SLOTS_PER_HISTORICAL_ROOT: Slot = 8192;
const SLOTS_PER_EPOCH: u64 = 8;

#[derive(PartialEq, Debug)]
pub enum Error {
    SlotOutOfRange,
    IndexOutOfRange,
    ApiError,
    AttestionPublishingError
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BeaconStateResponse {
    pub root: String,
    pub beacon_state: BeaconState<MinimalConfig>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutyInfo {
    pub validator_pubkey: String,
    pub attestation_slot: Slot,
    pub attestation_committee_index: CommitteeIndex,
    pub block_proposal_slot: Option<Slot>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutiesRequest {
    pub pubkeys: Vec<PublicKeyBytes>,
    pub epoch: Epoch
}

pub trait BeaconNode {
    fn get_state(&self) -> &BeaconState<MinimalConfig>;

    fn get_current_epoch(&self, state: &BeaconState<MinimalConfig>) -> Epoch;

    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    
    fn get_block_root(
        &self,
        state: &BeaconState<MinimalConfig>,
        epoch: Epoch,
    ) -> Result<H256, Error>;

    fn get_duties(
        &self,
        validators: Vec<PublicKeyBytes>,
        epoch: Epoch,
    ) -> Vec<DutyInfo>;

    fn publish_attestation(
        &self,
        attestation: Attestation<MinimalConfig>
    ) -> Result<(), Error>;

    fn get_block_root_at_slot(
        &self,
        state: &BeaconState<MinimalConfig>,
        slot: Slot,
    ) -> Result<H256, Error>;

    fn get_domain(
        &self,
        state: &BeaconState<MinimalConfig>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain;
}

#[derive(Clone)]
pub struct BasicBeaconNode {
    pub beacon_node_rest_client: Rc<RestClient>,
    last_known_state: BeaconState<MinimalConfig>,
}

impl BasicBeaconNode {
    pub fn new() -> BasicBeaconNode {
        let beacon_node_rest_client = Rc::new(RestClient::new(String::from("http://localhost:5052")).unwrap());
        let state: BeaconStateResponse = beacon_node_rest_client.get(&"/beacon/state").unwrap();
        BasicBeaconNode {
            beacon_node_rest_client,
            last_known_state: state.beacon_state,
        }
    }

    pub fn update_state(&mut self) -> () {
        let client = &self.beacon_node_rest_client;
        match client.get(&"/beacon/state") {
            Some(state) => self.last_known_state = state,
            None => println!("failed update state"),
        };
    }
}

impl BeaconNode for BasicBeaconNode {
    fn get_state(&self) -> &BeaconState<MinimalConfig> {
        &self.last_known_state
    }

    fn get_current_epoch(&self, state: &BeaconState<MinimalConfig>) -> Epoch {
        state.slot / 8
    }

    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot {
        epoch * 8
    }

    fn get_duties(&self, validators: Vec<PublicKeyBytes>, epoch: Epoch) -> Vec<DutyInfo> {
        let request_body = Option::Some(DutiesRequest {
            pubkeys: validators,
            epoch
        });
        (&self).beacon_node_rest_client.post("/validator/duties", request_body).unwrap()
    }

    fn publish_attestation(&self, request_body: Attestation<MinimalConfig>) -> Result<(), Error> {
        match (&self).beacon_node_rest_client.post("/validator/attestation", Option::Some(request_body)) {
            Some(()) => Ok(()),
            _ => Err(Error::AttestionPublishingError)
        }
    }

    fn get_block_root(
        &self,
        state: &BeaconState<MinimalConfig>,
        epoch: Epoch,
    ) -> Result<H256, Error> {
        let slot: Slot = epoch * SLOTS_PER_EPOCH;
        self.get_block_root_at_slot(state, slot)
    }

    fn get_block_root_at_slot(
        &self,
        state: &BeaconState<MinimalConfig>,
        slot: Slot,
    ) -> Result<H256, Error> {
        if slot < state.slot && state.slot <= slot + SLOTS_PER_HISTORICAL_ROOT {
            let root_idx = (slot % SLOTS_PER_HISTORICAL_ROOT) as usize;
            return Ok(state.historical_roots[root_idx]);
        }
        Err(Error::SlotOutOfRange)
    }

    fn get_domain(
        &self,
        state: &BeaconState<MinimalConfig>,
        domain_type: DomainType,
        message_epoch: Option<Epoch>,
    ) -> Domain {

        let epoch = match message_epoch {
            Some(epoch) => epoch,
            None => state.fork.epoch
        };

        let mut result = (domain_type as u64) << 8;

        let version = if epoch < state.fork.epoch {
            state.fork.previous_version.clone()
        } else {
            state.fork.current_version.clone()
        };
        for byte in version {
            result = (result | (byte as u64) ) << 2; 
        }
        result
    }
}
