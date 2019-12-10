use crate::attestation_producer::AttestationProducer;
use crate::beacon_node::{BasicBeaconNode, BeaconNode, DutyInfo};
use types::config::Config as EthConfig;
use types::primitives::{CommitteeIndex, Epoch, ValidatorIndex};
use std::{thread, time};
use bls::PublicKeyBytes;
use hex;

const SLOTS_PER_EPOCH: u64 = 8;

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    beacon_node: BasicBeaconNode,
    validators: Vec<(PublicKeyBytes, ValidatorIndex, String)>,
    attestation_producer: AttestationProducer<C>,
}

impl<C: EthConfig> ValidatorService<C> {
    pub fn new(eth_config: C, validators_keys: Vec<String>) -> ValidatorService<C> {
        let validators = parse_validators(validators_keys).unwrap();
        let attestation_producer = AttestationProducer {
            config: eth_config,
            beacon_node: BasicBeaconNode::new(),
        };
        ValidatorService {
            eth_config,
            beacon_node: BasicBeaconNode::new(),
            validators,
            attestation_producer
        }
    }

    pub fn start(&self) {
        let mut counter = 0u128;

        loop {
            println!("Fetching current beacon state...");
            let beacon_state = self.beacon_node.get_state();
            let epoch: Epoch = beacon_state.fork.epoch;
            let mut validator_pubkeys = Vec::new();

            for validator in &self.validators {
                validator_pubkeys.push(validator.0.clone());
            }

            let duties = self.beacon_node.get_duties(validator_pubkeys, epoch);
            let mut current_slot = beacon_state.slot % SLOTS_PER_EPOCH;
            loop {
                println!("Work at slot: {}", current_slot);
                for duty in duties.iter() {
                    if duty.attestation_slot == current_slot {
                        println!("\tvalidator {} should attest block", duty.validator_pubkey);
                        let attestation_data = match self.get_validator_index(&duty.validator_pubkey) {
                            Some(validator_index) => self.attestation_producer.get_attestation_data(
                                &beacon_state, duty.attestation_committee_index, validator_index),
                            _ => None
                        };
                        match attestation_data {
                            Some(data) => self.beacon_node.publish_attestation(data).unwrap(),
                            None => println!("Failed to build attestation data, for validator: {}", duty.validator_pubkey)
                        }
                        
                    }
                    match duty.block_proposal_slot {
                        Some(slot) => {
                            if slot == current_slot {
                                println!("\t\tvalidator {} should propose block", duty.validator_pubkey);
                            }
                        },
                        _ => ()
                    };
                }
                let slot_duration = time::Duration::from_millis(12000);
                thread::sleep(slot_duration);
                current_slot = current_slot + 1;
                if current_slot > SLOTS_PER_EPOCH {
                    break;
                }
            }
            // let duty_info = duties.first().unwrap();
            // println!("Duty fetched, will be working on slot: {}", duty_info.attestation_slot);
            // let job = self.calculate_job(duty_info);
            
            // match job {
            //     WorkInfo::Attest => {
            //         println!("Attesting...");

            

            //         let commitee_index: CommitteeIndex = 1;
            //         let attestation_data = attestation_producer.construct_attestation_data(
            //             beacon_state,
            //             beacon_state.slot,
            //             commitee_index,
            //         );

            //         let attestation = attestation_producer.construct_attestation(
            //             beacon_state,
            //             attestation_data,
            //             beacon_state.slot,
            //             commitee_index,
            //             0, //TODO: use real index
            //         );

            //         println!("Attestation result: {}", attestation.is_some());

            //         //self.beacon_node.publish_attestation(attestation);
            //     }
            //     WorkInfo::SignBlock => println!("Producing..."),
            //     WorkInfo::None => println!("No work."),
            // }
            
            counter = counter + 1;
            if counter > 65 {
                break;
            }
        }
        &self.end();
    }

    fn end(&self) {
        println!("End service work.");
    }

    fn get_validator_index(&self, pubkey: &String) -> Option<ValidatorIndex> {
        for validator in &self.validators {
            if validator.2 == *pubkey {
                return Some(validator.1);
            }
        }
        None
    }
}

fn parse_validators(pubkeys: Vec<String>) -> Result<Vec<(PublicKeyBytes, ValidatorIndex, String)>, String> {
    const PREFIX: &str = "0x";
    let mut result = Vec::new();
    for index in 0..pubkeys.len()  {
        if pubkeys[index].starts_with(PREFIX) {
            let pubkey_bytes = hex::decode(pubkeys[index].trim_start_matches(PREFIX)).unwrap();
            let pubkey = PublicKeyBytes::from_bytes(pubkey_bytes.as_slice()).unwrap();
            result.push((pubkey, index as u64, pubkeys[index].to_owned()));
        } else {
            return Err(String::from( "Public key must have a 0x prefix"))
        }
    }
    Ok(result)
}
