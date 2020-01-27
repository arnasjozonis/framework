use crate::rest_client::RestClient;
use bls::PublicKeyBytes;
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use types::beacon_state::BeaconState;
use types::config::MinimalConfig;
use types::primitives::{CommitteeIndex, Domain, DomainType, Epoch, Slot, ValidatorIndex, H256};
use types::types::{Attestation, BeaconBlock};

const SLOTS_PER_HISTORICAL_ROOT: Slot = 8192;
const SLOTS_PER_EPOCH: u64 = 8;

#[derive(PartialEq, Debug)]
pub enum Error {
    SlotOutOfRange,
    IndexOutOfRange,
    ApiError,
    AttestionPublishingError,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BeaconStateResponse {
    pub root: String,
    pub beacon_state: Option<BeaconState<MinimalConfig>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutyInfo {
    pub validator_pubkey: String,
    pub attestation_slot: Slot,
    pub attestation_committee_index: CommitteeIndex,
    pub attestation_committee_position: ValidatorIndex,
    pub block_proposal_slot: Option<Slot>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutiesRequest {
    pub pubkeys: Vec<PublicKeyBytes>,
    pub epoch: Epoch,
}

pub trait BeaconNode {
    fn get_state(&self) -> &Option<BeaconState<MinimalConfig>>;

    fn get_current_epoch(&self, state: &BeaconState<MinimalConfig>) -> Epoch;

    fn compute_start_slot_at_epoch(&self, epoch: Epoch) -> Slot;
    fn get_block_root(
        &self,
        state: &BeaconState<MinimalConfig>,
        epoch: Epoch,
    ) -> Result<H256, Error>;

    fn get_duties(&self, validators: Vec<PublicKeyBytes>, epoch: Epoch) -> Vec<DutyInfo>;

    fn publish_attestation(&self, attestation: Attestation<MinimalConfig>) -> Result<(), Error>;

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

    fn get_block(&self, slot: Slot, root: String) -> Option<BeaconBlock<MinimalConfig>>;
}

#[derive(Clone)]
pub struct BasicBeaconNode {
    pub beacon_node_rest_client: Rc<RestClient>,
    last_known_state: Option<BeaconState<MinimalConfig>>,
}

impl BasicBeaconNode {
    pub fn new() -> BasicBeaconNode {
        let beacon_node_rest_client =
            Rc::new(RestClient::new(String::from("http://localhost:5052")).unwrap());
        let state: Option<BeaconStateResponse> = beacon_node_rest_client.get(&"/beacon/state");
        let last_known_state = match state {
            Some(state_response) => state_response.beacon_state,
            None => None,
        };
        BasicBeaconNode {
            beacon_node_rest_client,
            last_known_state,
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
    fn get_state(&self) -> &Option<BeaconState<MinimalConfig>> {
        &self.last_known_state
    }

    fn get_block(&self, slot: Slot, root: String) -> Option<BeaconBlock<MinimalConfig>> {
        let url = format!("/validator/block?slot={}&root={}", slot, root);
        (&self).beacon_node_rest_client.get(&url[..])
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
            epoch,
        });
        (&self)
            .beacon_node_rest_client
            .post("/validator/duties", request_body)
            .unwrap()
    }

    fn publish_attestation(&self, request_body: Attestation<MinimalConfig>) -> Result<(), Error> {
        match (&self)
            .beacon_node_rest_client
            .post("/validator/attestation", Option::Some(request_body))
        {
            Some(()) => Ok(()),
            _ => Err(Error::AttestionPublishingError),
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
        let mut bytes: Vec<u8> = int_to_bytes4(domain_type);
        let epoch = match message_epoch {
            Some(epoch) => epoch,
            None => state.fork.epoch,
        };
        let mut version = if epoch < state.fork.epoch {
            state.fork.previous_version.clone().to_vec()
        } else {
            state.fork.current_version.clone().to_vec()
        };
        bytes.append(&mut version);
        let mut fork_and_domain = [0; 8];
        fork_and_domain.copy_from_slice(&bytes);

        u64::from_le_bytes(fork_and_domain)
    }
}

pub fn int_to_bytes4(int: u32) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(4);
    bytes.put_u32_le(int);
    bytes.to_vec()
}
