//!
//! # Proxy Options
//! 
//! Sub-module: `crate::proxy::mode`
//! 
//! Defines Proxy Options and Modes.
//! 

// Importing ProxyModeOptions enum
pub mod mode;

use serde::{Deserialize, Serialize};
use mode::ProxyModeOptions;

///
/// `ProxyOptions` struct. Defines structure of emitted JSON.
///
/// See <https://docs.microsoft.com/en-us/deployedge/microsoft-edge-policies#proxysettings>
/// for the full description of these options.
/// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyOptions {
    /// The Proxy Mode.
    pub proxy_mode: String,
    /// A valid URL to a .pac file for Proxy Configuration.
    pub proxy_pac_url: String,
    /// URL of Proxy Server.
    pub proxy_server: String,
    /// A list of URIs that bypass the configured Proxy Server.
    pub proxy_bypass_list: String,
}

impl ProxyOptions {
    ///
    /// Initializes a new `ProxyOptions` struct given supplied values.
    /// 
    pub fn new(
        mode: ProxyModeOptions,
        proxy_pac_url: String,
        proxy_server: String,
        proxy_bypass_list: String ) -> Self 
    {        
        let pmode: String;
        let pac: String;
        let pserver: String;
        let pbypass: String;

        match mode {
            ProxyModeOptions::Direct => {
                pmode = "direct".to_string();
            },
            ProxyModeOptions::System => {
                pmode = "system".to_string();
            },
            ProxyModeOptions::AutoDetect => {
                pmode = "auto_detect".to_string();
            },
            ProxyModeOptions::FixedServers => {
                pmode = "fixed_servers".to_string();
            },
            ProxyModeOptions::PacUrl => {
                pmode = "pac_url".to_string();
            },
        };

        match mode {
            ProxyModeOptions::Direct
            | ProxyModeOptions::System
            | ProxyModeOptions::AutoDetect => {
                pac = "".to_string();
                pserver = "".to_string();
                pbypass = "".to_string();
            },
            ProxyModeOptions::FixedServers => {
                pac = "".to_string();
                pserver = proxy_server;
                pbypass = proxy_bypass_list;
            },
            ProxyModeOptions::PacUrl => {
                pac = proxy_pac_url;
                pserver = "".to_string();
                pbypass = "".to_string();
            },
        };

        Self {
            proxy_mode: pmode,
            proxy_pac_url: pac,
            proxy_server: pserver,
            proxy_bypass_list: pbypass,
        }
    }
}