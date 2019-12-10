use crate::rest_client::RestClient;
use serde::{Deserialize, Serialize};
use types::beacon_state::BeaconState;
use types::config::{Config, MinimalConfig};
use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, ValidatorIndex, H256};
use types::types::Attestation;
use std::rc::Rc;
use bls::PublicKeyBytes;


const EMPTY_BODY: Option<bool> = Option::None;

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
    fn get_beacon_committee(
        &self,
        state: &BeaconState<MinimalConfig>,
        slot: Slot,
        index: CommitteeIndex,
    ) -> Vec<ValidatorIndex>;
    fn get_block_root(
        &self,
        state: &BeaconState<MinimalConfig>,
        epoch: Epoch,
    ) -> Result<H256, Error>;

    fn get_duty(
        &self,
        epoch: Epoch,
    ) -> Vec<DutyInfo>;

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
    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot {
        epoch * 8
    }
    
    fn get_duty(&self, epoch: Epoch) -> Vec<DutyInfo> {
        (&self).beacon_node_rest_client.get("/validator/duties?validator_pubkeys=0x88c141df77cd9d8d7a71a75c826c41a9c9f03c6ee1b180f3e7852f6a280099ded351b58d66e653af8e42816a4d8f532e&epoch=0").unwrap()
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
        Ok(H256::from([0; 32]))
    }
    fn get_block_root_at_slot(
        &self,
        state: &BeaconState<MinimalConfig>,
        slot: Slot,
    ) -> Result<H256, Error> {
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
