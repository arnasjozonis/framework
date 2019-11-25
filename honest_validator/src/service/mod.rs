use types::config::{ Config as EthConfig };
use types::primitives::{Epoch, ValidatorIndex};
use crate::duties_manager::{ DutiesManager, WorkInfo, TestWorker, Worker };
use crate::beacon_node::{BasicBeaconNode, BeaconNode};

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    beacon_node: BasicBeaconNode

}

impl<C: EthConfig> ValidatorService<C> {
    pub fn new(eth_config: C) -> ValidatorService<C> {
        let beacon_node = BasicBeaconNode::new();
        ValidatorService { eth_config, beacon_node }
    }

    pub fn start(&self) {
        println!("Start service work.");
        let beacon_state = self.beacon_node.get_state();
        println!("{}", beacon_state.slot);
        let epoch: Epoch = 0;
        let validator_index: ValidatorIndex = 1;
        let job = match DutiesManager::get_duty(&beacon_state, epoch, validator_index, &self.beacon_node) {
            Ok(job) => {
                println!("Got job...");
                job
            },
            Err(num) => {
                println!{"Error code: {}", num};
                WorkInfo::None
            }
        };
        match job {
            WorkInfo::Attest => {
                let worker = TestWorker {};
                println!("Attesting...");
                println!("Attestation result: {}", worker.do_work(&beacon_state).unwrap())
            },
            WorkInfo::SignBlock => println!("Producing..."),
            WorkInfo::None => println!("No work.")
        }
        
        &self.end();
    }

    pub fn end(&self) {
        println!("End service work.");
    }
}