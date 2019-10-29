use types::config::Config;

pub enum WorkInfo {
    SignBlock,
    Attest,
    None
}

pub struct DutiesManager<C: Config> {
    pub config: C
    //pub validator: Validator,
    //pub beacon_node: BeaconNode
}

impl<C: Config> DutiesManager<C> {
    pub fn get_duty() -> WorkInfo {
        WorkInfo::None
    }
}