use crate::attestation_producer::AttestationProducer;
use crate::beacon_node::{BasicBeaconNode, BeaconNode, Error};
use bls::{PublicKeyBytes, SecretKey};
use hex;
use std::{thread, time};
use types::config::Config as EthConfig;
use types::primitives::{Epoch, ValidatorIndex};
use serde::{Deserialize};

const SLOTS_PER_EPOCH: u64 = 8;

#[derive(Deserialize)]
pub struct KeysPair {
    private: String,
    public: String
}

pub struct Validator {
    public_key: PublicKeyBytes,
    index: ValidatorIndex,
    public_key_str: String,
    private_key: SecretKey
}

pub struct Service<C: EthConfig> {
    beacon_node: BasicBeaconNode,
    validators: Vec<Validator>,
    attestation_producer: AttestationProducer<C>,
}

impl<C: EthConfig> Service<C> {
    pub fn new(eth_config: C, validators_keys: Vec<KeysPair>) -> Service<C> {
        let validators = parse_validators(validators_keys).unwrap();
        let attestation_producer = AttestationProducer {
            config: eth_config,
            beacon_node: BasicBeaconNode::new(),
        };
        Service {
            beacon_node: BasicBeaconNode::new(),
            validators,
            attestation_producer,
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
                validator_pubkeys.push(validator.public_key.clone());
            }

            let duties = self.beacon_node.get_duties(validator_pubkeys, epoch);
            let mut current_slot = beacon_state.slot % SLOTS_PER_EPOCH;
            loop {
                println!("Work at slot: {}", current_slot);
                for duty in duties.iter() {
                    if duty.attestation_slot == current_slot {
                        let attestation = match self.get_validator_index(&duty.validator_pubkey) {
                            Some(validator_index) => {
                                println!("\tvalidator {} should attest block", validator_index);
                                let private_key = (&self).get_private_key(validator_index);
                                self.attestation_producer.get_attestation(
                                    &beacon_state,
                                    duty.attestation_committee_index,
                                    duty.attestation_committee_position,
                                    private_key
                                )
                            }
                            _ => None,
                        };

                        let attestation_result = match attestation {
                            Some(data) => self.beacon_node.publish_attestation(data),
                            None => {
                                println!(
                                    "Failed to build attestation data, for validator: {}",
                                    duty.validator_pubkey
                                );
                                Err(Error::AttestionPublishingError)
                            }
                        };
                        match attestation_result {
                            Err(e) => match e {
                                Error::AttestionPublishingError => {
                                    println!("Attestation publishing error in API")
                                }
                                _ => println!("Unknown error in API"),
                            },
                            _ => (),
                        }
                    }
                    match duty.block_proposal_slot {
                        Some(slot) => {
                            if slot == current_slot {
                                println!("\n\n");
                                println!(
                                    "\t\tvalidator {} should propose block",
                                    duty.validator_pubkey
                                );
                                match (&self)
                                    .beacon_node
                                    .get_block(slot, String::from("tetatata"))
                                {
                                    Some(_) => println!("block received"),
                                    None => println!("block not received"),
                                };
                                println!("\n\n");
                            }
                        }
                        _ => (),
                    };
                }
                let slot_duration = time::Duration::from_millis(12000);
                thread::sleep(slot_duration);
                current_slot = current_slot + 1;
                if current_slot > SLOTS_PER_EPOCH {
                    break;
                }
            }
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
            if validator.public_key_str == *pubkey {
                return Some(validator.index.clone());
            }
        }
        None
    }

    fn get_private_key(&self, validator_index: ValidatorIndex) -> SecretKey {
        let mut privkey: SecretKey = SecretKey::random();
        for v in &self.validators {
            if v.index == validator_index {
                privkey = v.private_key.clone();
                break;
            }
        }
        privkey
    }
}

fn parse_validators(
    keys: Vec<KeysPair>,
) -> Result<Vec<Validator>, String> {
    const PREFIX: &str = "0x";
    let mut result = Vec::new();
    for index in 0..keys.len() {
        let public = &keys[index].public;
        let private = &keys[index].private;
        if public.starts_with(PREFIX) && private.starts_with(PREFIX) {
            let pubkey_bytes = hex::decode(public.trim_start_matches(PREFIX)).unwrap();
            let public_key = PublicKeyBytes::from_bytes(pubkey_bytes.as_slice()).unwrap();
            let private_key_bytes = hex::decode(private.trim_start_matches(PREFIX)).unwrap();
            
            let mut bytes = vec![0; 48 - private_key_bytes.len()];
            bytes.extend_from_slice(&private_key_bytes[..]);
            let private_key = SecretKey::from_bytes(&bytes)
                .map_err(|e| format!("Failed to decode bytes into secret key: {:?}", e))?;
            result.push(Validator {
                private_key,
                public_key,
                index: index as ValidatorIndex,
                public_key_str: public.to_owned(),
            });
        } else {
            return Err(String::from("Public key must have a 0x prefix"));
        }
    }
    Ok(result)
}
