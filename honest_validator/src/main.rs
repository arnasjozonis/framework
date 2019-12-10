extern crate framework_honest_validator as hv;

use clap::{App, Arg};
use hv::service::ValidatorService;
use types::config::MinimalConfig;
use bls::PublicKeyBytes;

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
    let validator = PublicKeyBytes::from_bytes(
    &[  
        0x86,
        0xa7,
        0x38,
        0x86,
        0xaa,
        0x01,
        0x14,
        0xbb,
        0xdb,
        0xba,
        0x34,
        0x6c,
        0xb7,
        0xc0,
        0x73,
        0x76,
        0xc8,
        0x1b,
        0x54,
        0x9a,
        0x48,
        0x02,
        0xc2,
        0x4d,
        0x98,
        0xeb,
        0xbc,
        0x54,
        0xa6,
        0xa1,
        0xb5,
        0xd2,
        0xac,
        0x87,
        0x4e,
        0xf6,
        0x57,
        0xcf,
        0xb2,
        0x7c,
        0x36,
        0x44,
        0xfc,
        0xb8,
        0x5f,
        0x97,
        0xa2,
        0xb5 
    ]).unwrap();
    let service: ValidatorService<MinimalConfig> = ValidatorService::new(cfg, (validator, 0));
    service.start();
}
