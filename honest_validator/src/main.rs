extern crate framework_honest_validator as hv;

use types::config::{ QuickConfig as EthConfigQuick };
use hv::service::ValidatorService;
use hv::duties_manager::DutiesManager;
use hv::beacon_node::{BasicBeaconNode};
use clap::{App, Arg};
use hv::rest_client::{RestClient};

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
                .long("spec")
                .value_name("CONFIGURATION")
                .help("Specifies the default eth2 spec type.")
                .takes_value(true)
                .possible_values(&["mainnet", "minimal", "internal_test"])
        ).get_matches();

    let app_cfg = match matches.value_of("spec") {
        Some("internal_test") => AppConfiguration::InternalTest,
        _ => AppConfiguration::Unsupported,
    };

    let cfg = match app_cfg {
        AppConfiguration::InternalTest =>  EthConfigQuick,
        AppConfiguration::Unsupported =>  EthConfigQuick
    };
    let beacon_node = BasicBeaconNode {
    };
    let dm = DutiesManager::new(beacon_node);
    let service: ValidatorService<EthConfigQuick> = ValidatorService::new(dm, cfg);
    //service.start();
    let mut rest_api = RestClient::new(String::from("http://localhost:5052")).unwrap();
    let test = rest_api.get_beacon_validators();
    match test {
        Some(res) => println!("{}", res.first().unwrap().pubkey),
        _ => ()
    }
}
