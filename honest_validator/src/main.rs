extern crate framework_honest_validator as hv;

use clap::{App, Arg};
use hv::service::ValidatorService;
use types::config::MinimalConfig;
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

enum AppConfiguration {
    InternalTest,
    Unsupported,
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
                .possible_values(&["mainnet", "minimal", "internal_test"]),
        )
        .get_matches();

    let app_cfg = match matches.value_of("spec") {
        Some("internal_test") => AppConfiguration::InternalTest,
        _ => AppConfiguration::Unsupported,
    };

    let cfg = match app_cfg {
        AppConfiguration::InternalTest => MinimalConfig::default(),
        AppConfiguration::Unsupported => MinimalConfig::default(),
    };
    let mut validators = Vec::new();
    let file = File::open("honest_validator/mock_data/validator_pubkeys.txt").unwrap();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let validator_key = line.unwrap();
        validators.push(validator_key);
    };
    let service: ValidatorService<MinimalConfig> = ValidatorService::new(cfg, validators);
    service.start();
}
