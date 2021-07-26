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
        "",
        "",
        "",
    );
    
    println!("Proxy settings:\n {:?}", serde_json::to_string(&proxy)?);
    
    Ok(())
}