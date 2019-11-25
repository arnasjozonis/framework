extern crate framework_honest_validator as hv;

use types::config::{ QuickConfig };
use hv::service::ValidatorService;
use clap::{App, Arg};

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
        AppConfiguration::InternalTest =>  QuickConfig::default(),
        AppConfiguration::Unsupported =>  QuickConfig::default()
    };
    
    let service: ValidatorService<QuickConfig> = ValidatorService::new(cfg);
    service.start();
}
