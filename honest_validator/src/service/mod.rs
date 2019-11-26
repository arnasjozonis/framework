use crate::attestation_producer::AttestationProducer;
use crate::beacon_node::{BasicBeaconNode, BeaconNode};
use crate::duties_manager::{DutiesManager, TestWorker, WorkInfo, Worker};
use types::config::Config as EthConfig;
use types::primitives::{CommitteeIndex, Epoch, ValidatorIndex};

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    beacon_node: BasicBeaconNode,
}

impl<C: EthConfig> ValidatorService<C> {
    pub fn new(eth_config: C) -> ValidatorService<C> {
        ValidatorService {
            eth_config,
            beacon_node: BasicBeaconNode::new(),
        }
    }

    pub fn start(&self) {
        println!("Start service work. Fetchin current beacon state...");
        let beacon_state = self.beacon_node.get_state();
        println!(
            "State fetched: slot: {}, epoch: {}, genesis_time: {}", 
            beacon_state.slot, 
            beacon_state.fork.epoch, 
            beacon_state.genesis_time
        );
        let epoch: Epoch = 0;
        let validator_index: ValidatorIndex = 1;
        let job =
            match DutiesManager::get_duty(&beacon_state, epoch, validator_index, &self.beacon_node)
            {
                Ok(job) => {
                    println!("Got job...");
                    job
                }
                Err(num) => {
                    println! {"Error code: {}", num};
                    WorkInfo::None
                }
            };
        match job {
            WorkInfo::Attest => {
                println!("Attesting...");

                let worker = TestWorker {};
                println!("Worker result: {}", worker.do_work(&beacon_state).unwrap());

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
                    validator_index,
                );

                println!("Attestation result: {}", attestation.is_some());
            }
            WorkInfo::SignBlock => println!("Producing..."),
            WorkInfo::None => println!("No work."),
        }
        &self.end();
    }

    pub fn end(&self) {
        println!("End service work.");
    }
}
