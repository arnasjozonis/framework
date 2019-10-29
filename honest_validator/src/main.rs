extern crate framework_honest_validator as hv;

use types::config::{ Config as EthConfigType, QuickConfig as EthConfigQuick };
use hv::service::ValidatorService;
use hv::duties_manager::DutiesManager;

fn main() {
    println!("Honest validator says hello!");
    let cfg = EthConfigQuick;
    let dm = DutiesManager { config: cfg };
    ValidatorService::start(dm, cfg);
}
