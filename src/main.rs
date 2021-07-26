#![allow(dead_code)]

pub mod proxy;

extern crate serde; // 1.0.126
extern crate serde_json; // 1.0.64

use serde_json::Result;

use proxy::mode::ProxyModeOptions;
use proxy::ProxyOptions;

pub fn main() -> Result<()> {
    let proxy: ProxyOptions = ProxyOptions::new(
        ProxyModeOptions::Direct,
        "127.0.1.1:8090/sun.pac",
        "127.0.0.1:8080",
        "192.168.1.0/24",
    );
    
    println!("Proxy settings:\n{}", serde_json::to_string_pretty(&proxy)?);
    
    Ok(())
}