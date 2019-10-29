use types::config::{ Config as EthConfig};
use crate::duties_manager::DutiesManager;

pub struct ValidatorService<C: EthConfig> {
    eth_config: C,
    duties_manager: DutiesManager<C>
    //beacon_client: BeaconClient<C>,

}

impl<C: EthConfig> ValidatorService<C> {
    fn init(duties_manager: DutiesManager<C>, eth_config: C) -> ValidatorService<C> {
        ValidatorService { duties_manager, eth_config }
    }

    pub fn start(duties_manager: DutiesManager<C>, eth_config: C) {
        let service = ValidatorService::init(duties_manager, eth_config);
        println!("Start service work.");
        service.end();
    }

    pub fn end(&self) {
        println!("End service work.");
    }
}