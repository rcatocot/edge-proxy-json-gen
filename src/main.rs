#![allow(dead_code)]

extern crate clap; // 2.33.3
extern crate serde; // 1.0.126
extern crate serde_json; // 1.0.64

pub mod args;
pub mod proxy;

use serde_json::Result;

use proxy::mode::ProxyModeOptions;
use proxy::ProxyOptions;

pub fn main() -> Result<()> {

    let matches = args::build_args().get_matches();

    let mode = match matches.value_of("mode").unwrap() {
        "direct" => ProxyModeOptions::Direct,
        "system" => ProxyModeOptions::System,
        "auto_detect" => ProxyModeOptions::AutoDetect,
        "fixed_servers" => ProxyModeOptions::FixedServers,
        "pac_url" => ProxyModeOptions::PacUrl,
        _ => panic!("Unknown proxy mode!")
    };

    let proxy: ProxyOptions = ProxyOptions::new(
        mode,
        matches.value_of("pac_url").unwrap_or_default().to_string(),
        matches.value_of("proxy-server").unwrap_or_default().to_string(),
        matches.value_of("proxy-bypass-list").unwrap_or_default().to_string(),
    );
    
    println!("Proxy settings:\n");

    match matches.is_present("pretty-print") {
        true => println!("{}", serde_json::to_string_pretty(&proxy)?),
        false => println!("{}", serde_json::to_string(&proxy)?),
    }
    
    Ok(())
}