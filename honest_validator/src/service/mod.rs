use types::config::{ Config as EthConfig };
use types::primitives::{Epoch, ValidatorIndex};
use crate::beacon_node::{BeaconState};
use crate::duties_manager::{ DutiesManager, WorkInfo, TestWorker, Worker };

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    duties_manager: DutiesManager
    //beacon_client: BeaconClient<C>,

}

impl<C: EthConfig> ValidatorService<C> {
    pub fn new(duties_manager: DutiesManager, eth_config: C) -> ValidatorService<C> {
        ValidatorService { duties_manager, eth_config }
    }

    pub fn start(&self) {
        println!("Start service work.");
        let beacon_state = BeaconState::default();
        let epoch: Epoch = 0;
        let validator_index: ValidatorIndex = 1;
        let job = match self.duties_manager.get_duty(&beacon_state, epoch, validator_index) {
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