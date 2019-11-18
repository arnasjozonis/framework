use types::config::{ Config as EthConfig, QuickConfig};
use types::beacon_state::{ BeaconState };
use types::primitives::{Epoch, ValidatorIndex};
use crate::duties_manager::DutiesManager;
use crate::beacon_node::{BasicBeaconNode};

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
        let beacon_state: BeaconState<QuickConfig> = BeaconState::default();
        let epoch: Epoch = 0;
        let validator_index: ValidatorIndex = 1;
        match self.duties_manager.get_duty(&beacon_state, epoch, validator_index) {
            Ok(Workinfo) => println!("It works..."),
            Err(num) => println!{"Error code: {}", num}
        };
        &self.end();
    }

    pub fn end(&self) {
        println!("End service work.");
    }
}