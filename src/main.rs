//!
//! # Edge Proxy Mode Formatter
//! 
//! This is a simple rust CLI app for generating valid JSON
//! in accordance to Microsoft Edge Group Policy options for
//! Proxy Settings.
//! 
//! See <https://docs.microsoft.com/en-us/deployedge/microsoft-edge-policies#proxysettings>
//! for the full explanation of proxy options for Edge GPOs.
//! 
//! # Usage
//! ```text
//! USAGE:
//!    edge-proxy-json-gen.exe [FLAGS] [OPTIONS] <mode>
//!
//!    FLAGS:
//!         -h, --help       Prints help information
//!         -f, --pretty     Emit formatted 'pretty' JSON.
//!         -V, --version    Prints version information

//!    OPTIONS:
//!         -u, --pac-url <pac-url>                  URL containing proxy .pac file.
//!         -b, --bypass-list <proxy-bypass-list>    List of IP addresses to bypass proxy.
//!         -s, --server <proxy-server>              URI of proxy server including port.

//!    ARGS:
//!         <mode>    Proxy Mode.
//! ```
//! 

#![allow(dead_code)]

extern crate clap; // 2.33.3
extern crate serde; // 1.0.126
extern crate serde_json; // 1.0.64

pub mod args;
pub mod proxy;

use serde_json::Result;

use proxy::mode::ProxyModeOptions;
use proxy::ProxyOptions;

///
/// # Main Program Function
/// 
/// Parses arguments via [`clap`](clap) helpers.
/// 
/// Emits JSON in accordance to passed arguments.
/// 
/// # Panics
/// This function will panic if:
/// * There is a JSON Serialization error ([`serde_json::error::Error`](serde_json::error::Error))
/// * An invalid mode is given.
/// 
pub fn main() -> Result<()> {
    // Build argument matches from `crate::args` module.
    let matches = args::build_args().get_matches();

    // Map user input to `ProxyModeOptions` enum.
    // Panic! if any other value given.
    let mode = match matches.value_of("mode").unwrap() {
        "direct" => ProxyModeOptions::Direct,
        "system" => ProxyModeOptions::System,
        "auto_detect" => ProxyModeOptions::AutoDetect,
        "fixed_servers" => ProxyModeOptions::FixedServers,
        "pac_url" => ProxyModeOptions::PacUrl,
        _ => panic!("Unknown proxy mode!")
    };

    // Build `crate::proxy::ProxyOptions` struct from values
    // given by the user.
    let proxy: ProxyOptions = ProxyOptions::new(
        mode,
        matches.value_of("pac_url").unwrap_or_default().to_string(),
        matches.value_of("proxy-server").unwrap_or_default().to_string(),
        matches.value_of("proxy-bypass-list").unwrap_or_default().to_string(),
    );
    
    println!("Proxy settings:\n");

    // If the 'pretty-print' argument is given, then use
    // `serde_json`s ability to pretty-print. Else, just emit
    // as normal.
    match matches.is_present("pretty-print") {
        true => println!("{}", serde_json::to_string_pretty(&proxy)?),
        false => println!("{}", serde_json::to_string(&proxy)?),
    }
    
    Ok(())
}