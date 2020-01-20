use crate::attestation_producer::AttestationProducer;
use crate::block_producer::{produce_block};
use crate::beacon_node::{BasicBeaconNode, BeaconNode, Error};
use bls::{PublicKeyBytes, SecretKey};
use hex;
use std::{thread, time};
use types::config::{Config as EthConfig};
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

    pub fn start(&self) -> Result<(), String>{
        let mut counter = 0u128;

        loop {
            println!("Fetching current beacon state...");
            let beacon_state = match &(self.beacon_node.get_state()) {
                Some(state) => state,
                None => return Err(String::from("can not get beacon state"))
            };
            let epoch: Epoch = beacon_state.fork.epoch;
            let mut validator_pubkeys = Vec::new();

            for validator in &self.validators {
                validator_pubkeys.push(validator.public_key.clone());
            }

            let duties = self.beacon_node.get_duties(validator_pubkeys, epoch);
            let mut current_slot = beacon_state.slot % SLOTS_PER_EPOCH;
            loop {
                println!("\n##################\nWork at slot: {}\n##################\n", current_slot);
                for duty in duties.iter() {
                    if duty.attestation_slot == current_slot {
                        let validator_index = self.get_validator_index(&duty.validator_pubkey).unwrap();
                        let private_key = (&self).get_private_key(validator_index);
                        let test = private_key.clone();
                        print_privates(test, &validator_index);
        
                        
                        println!("\n\tvalidator {} should attest block\n\t", validator_index);
                        
                        println!("\n\n");
                        let attestation = self.attestation_producer.get_attestation(
                            &beacon_state,
                            duty.attestation_committee_index,
                            duty.attestation_committee_position,
                            private_key
                        );

                        let attestation_result = match attestation {
                            Some(data) => self.beacon_node.publish_attestation(data),
                            None => {
                                println!(
                                    "Failed to build attestation data, for validator: {}",
                                    validator_index
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
                                let validator_index = self.get_validator_index(&duty.validator_pubkey).unwrap();
                                let private_key = (&self).get_private_key(validator_index);
                                //produce_block(&self.beacon_node, beacon_state, private_key, slot);
                                println!("\n\n");
                                println!(
                                    "\t\tvalidator {} should propose block",
                                    validator_index
                                );
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
        Ok(())
    }

    fn end(&self) {
        println!("End service work.");
    }

    fn get_validator_index(&self, pubkey: &String) -> Option<ValidatorIndex> {
        println!("{}", pubkey);
        for validator in &self.validators {
            if validator.public_key_str == *pubkey {
                println!("{}\n", validator.index.clone());
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

            for i in 0..bytes.len() {
                print!("{} ", bytes[i]);
            }
            println!("\n\n");
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

fn print_privates(pk: SecretKey, vi: &ValidatorIndex) {
    if *vi != 9 {
        return ();
    }
    let test_val: &str = r#"
        [{
            "private":"0x2b3b88a041168a1c4cd04bdd8de7964fd35238f95442dc678514f9dadb81ec34",
            "public":"0x9893413c00283a3f9ed9fd9845dda1cea38228d22567f9541dccc357e54a2d6a6e204103c92564cbc05f4905ac7c493a"
          }
        ]"#;
    let keys: Vec<KeysPair> = serde_json::from_str(test_val).unwrap();
    let v2 = &parse_validators(keys).unwrap()[0];
    let pk2 = v2.private_key.clone().as_raw().x.w;

    let pk = pk.as_raw().x.w;
    let mut pk_bytes: Vec<u8> = Vec::new();
    assert!(pk.len() == pk2.len());
    for i in 0..pk.len() {
        let pkb = pk[i].to_be_bytes();
        let pkb2 = pk2[i].to_be_bytes();
        for idx in 0..pkb.len() {
            pk_bytes.push(pkb[idx]);
            pk_bytes.push(pkb2[idx]);
        }
    }
    
    for i in 0..pk_bytes.len() {
        print!("{} ", pk_bytes[i]);
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use types::config::{MinimalConfig};
    use bls::{Signature, PublicKey};
    use types::primitives::{Domain};

    const VALIDATORS: &str = r#"
        [{
            "private":"0x25295f0d1d592a90b333e26e85149708208e9f8e8bc18f6c77bd62f8ad7a6866",
            "public":"0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c"
          },
          {
            "private":"0x51d0b65185db6989ab0b560d6deed19c7ead0e24b9b6372cbecb1f26bdfad000",
            "public":"0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b"
        }]"#;

        const INVALID_VALIDATORS: &str = r#"
        [{
            "private":"0x25295f0d1d592a90b333e26e85149708208e9f8e8bc18f6c77bd62f8ad7a6866",
            "public":"a99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c"
          },
          {
            "private":"51d0b65185db6989ab0b560d6deed19c7ead0e24b9b6372cbecb1f26bdfad000",
            "public":"0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b"
        }]"#;

    #[test]
    fn should_init_service() {
        let keys: Vec<KeysPair> = serde_json::from_str(VALIDATORS).unwrap();
        let service = Service::new(MinimalConfig::default(), keys);
        assert_eq!(service.validators[0].public_key_str, "0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c");
        assert_eq!(service.validators.len(), 2);
    }

    #[test]
    fn should_get_validator_index() {
        let keys: Vec<KeysPair> = serde_json::from_str(VALIDATORS).unwrap();
        let service = Service::new(MinimalConfig::default(), keys);
        let index = service.get_validator_index(&String::from("0xb89bebc699769726a318c8e9971bd3171297c61aea4a6578a7a4f94b547dcba5bac16a89108b6b6a1fe3695d1a874a0b"));
        assert_eq!(index, Some(1));
        let index = service.get_validator_index(&String::from("random"));
        assert_eq!(index, None);
    }

    #[test]
    fn should_init_validators() {
        let keys: Vec<KeysPair> = serde_json::from_str(VALIDATORS).unwrap();
        let parsed = parse_validators(keys).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].index, 0);
        assert_eq!(parsed[1].index, 1);
        let mut bytes = vec![0u8; 48];
        let private_key_other_bytes = hex::decode("51d0b65185db6989ab0b560d6deed19c7ead0e24b9b6372cbecb1f26bdfad000").unwrap();
        assert_eq!(private_key_other_bytes.len(), 32);
        for i in 16..48 {
            bytes[i] = private_key_other_bytes[i - 16];
        }

        assert_eq!(parsed[1].private_key, SecretKey::from_bytes(&bytes).unwrap());
    }

    #[test]
    fn should_validate_validators() {
        let keys: Vec<KeysPair> = serde_json::from_str(INVALID_VALIDATORS).unwrap();
        let optb = Vec::new();
        let parsed = parse_validators(keys).unwrap_or(optb);
        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn should_accept_signature_for_attestation() {
        let keys: Vec<KeysPair> = serde_json::from_str(VALIDATORS).unwrap();
        let parsed = parse_validators(keys).unwrap();
        let domain: Domain = 2; // attestation domain;
        let test_msg = [1u8, 2u8];
        let signature = Signature::new(&test_msg, domain, &(parsed[0].private_key));
        let pubkey_bytes = hex::decode(parsed[0].public_key_str.trim_start_matches("0x")).unwrap();
        let public_key = PublicKey::from_bytes(&pubkey_bytes).unwrap();
        let verification_result = signature.verify(&test_msg, domain, &public_key);
        assert!(verification_result);
    }
}