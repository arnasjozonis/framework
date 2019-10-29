extern crate framework_honest_validator;

use types::config::{ Config as EthConfigType, QuickConfig as EthConfigQuick };
use framework_honest_validator::service::ValidatorService;
use framework_honest_validator::duties_manager::DutiesManager;

fn main() {
    println!("Honest validator says hello!");
    let cfg = EthConfigQuick;
    let dm = DutiesManager { config: cfg };
    ValidatorService::start(dm, cfg);
}
