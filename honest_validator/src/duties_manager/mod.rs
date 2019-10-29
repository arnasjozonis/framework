use types::config::Config;
use crate::beacon_node::BeaconNode;

pub enum WorkInfo {
    SignBlock,
    Attest,
    None
}

pub struct DutiesManager<C: Config, BN: BeaconNode<C>> {
    pub config: C,
    //pub validator: Validator,
    pub beacon_node: BN
}

impl<C: Config, BN: BeaconNode<C>> DutiesManager<C, BN> {
    pub fn get_duty() -> WorkInfo {
        WorkInfo::None
    }
}