use crate::attestation_producer::AttestationProducer;
use crate::beacon_node::{BasicBeaconNode, BeaconNode, DutyInfo};
use types::config::Config as EthConfig;
use types::primitives::{CommitteeIndex, Epoch, ValidatorIndex};
use std::{thread, time};
use bls::PublicKeyBytes;

pub enum WorkInfo {
    SignBlock,
    Attest,
    None,
}

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    beacon_node: BasicBeaconNode,
    validator: (PublicKeyBytes, ValidatorIndex),
}

impl<C: EthConfig> ValidatorService<C> {
    pub fn new(eth_config: C, validator: (PublicKeyBytes, ValidatorIndex)) -> ValidatorService<C> {
        ValidatorService {
            eth_config,
            beacon_node: BasicBeaconNode::new(),
            validator
        }
    }

    pub fn start(&self) {
        let mut counter = 0u128;
        loop {
            println!("Fetching current beacon state...");
            let beacon_state = self.beacon_node.get_state();
            println!(
                "State fetched: slot: {}, epoch: {}, genesis_time: {}",
                beacon_state.slot, beacon_state.fork.epoch, beacon_state.genesis_time
            );
            let epoch: Epoch = 0;
            let duties = self.beacon_node.get_duty(epoch);
            let duty_info = duties.first().unwrap();
            println!("Duty fetched, will be working on slot: {}", duty_info.attestation_slot);
            let job = self.calculate_job(duty_info);
            
            match job {
                WorkInfo::Attest => {
                    println!("Attesting...");

                    let mut attestation_producer = AttestationProducer {
                        config: self.eth_config,
                        beacon_node: self.beacon_node.clone(),
                    };

                    let commitee_index: CommitteeIndex = 1;
                    let attestation_data = attestation_producer.construct_attestation_data(
                        beacon_state,
                        beacon_state.slot,
                        commitee_index,
                    );

                    let attestation = attestation_producer.construct_attestation(
                        beacon_state,
                        attestation_data,
                        beacon_state.slot,
                        commitee_index,
                        self.validator.1,
                    );

                    println!("Attestation result: {}", attestation.is_some());

                    //self.beacon_node.publish_attestation(attestation);
                }
                WorkInfo::SignBlock => println!("Producing..."),
                WorkInfo::None => println!("No work."),
            }
            let ten_millis = time::Duration::from_millis(6500);
            thread::sleep(ten_millis);
            counter = counter + 1;
            if counter > 10 {
                break;
            }
        }
        &self.end();
    }

    fn end(&self) {
        println!("End service work.");
    }

    fn calculate_job(&self, duty_info: &DutyInfo) -> WorkInfo {
        //TODO: calculate assignment

        // let next_epoch = beacon_state.fork.epoch + 1;
        // if next_epoch < epoch {
        //     return Err(String::from(
        //         "Epoch to request duties is too far in the future",
        //     ));
        // };

        // let start_slot: Slot = beacon_node.compute_start_slot_at_epoch(epoch);
        // let end_slot = <MinimalConfig as Config>::SlotsPerEpoch::to_u64() + &start_slot;
        // for slot in start_slot..end_slot {
        //     let committee_count = beacon_node.get_committee_count_at_slot(beacon_state, slot);
        //     for index in 0..committee_count {
        //         let committee = beacon_node.get_beacon_committee(beacon_state, slot, index);
        //         let assignment = committee.iter().find(|&&idx| idx == validator_index);
        //         return match assignment {
        //             Some(val) => Ok(WorkInfo::Attest),
        //             None => Ok(WorkInfo::None),
        //         };
        //     }
        // }
        WorkInfo::None
    }

}
