extern crate framework_honest_validator as hv;

use types::config::{ Config as EthConfigType, QuickConfig as EthConfigQuick };
use hv::service::ValidatorService;
use hv::duties_manager::DutiesManager;
use hv::beacon_node::{BeaconNode, BasicBeaconNode};
use clap::{App, Arg, ArgMatches, SubCommand};

enum AppConfiguration {
    InternalTest,
    Unsupported
}

fn main() {
    println!("Honest validator says hello!");
    let matches = App::new("Honest Validator Client")
        .version("0.1.0")
        .author("Arnas Jozonis, Aurintas Bubinas, Rasa Šmigelskytė")
        .about("Eth 2.0 Validator Client")
        .arg(
            Arg::with_name("spec")
                .short("s")
                .value_name("TITLE")
                .help("Specifies the default eth2 spec type.")
                .takes_value(true)
                .possible_values(&["mainnet", "minimal", "internal_test"])
        ).get_matches();

    let app_cfg = match matches.value_of("spec") {
        Some("internal_test") => AppConfiguration::InternalTest,
        _ => AppConfiguration::Unsupported,
    };

    match app_cfg {
        AppConfiguration::InternalTest => println!{"Let's do some work"},
        AppConfiguration::Unsupported => panic!("Only internal test configuration is supported")
    }

    let cfg = EthConfigQuick;
    let dm = DutiesManager { 
        config: cfg,
        beacon_node:  BasicBeaconNode::new(cfg)};
    ValidatorService::start(dm, cfg);
}
