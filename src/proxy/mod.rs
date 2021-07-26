pub mod mode;

use serde::{Deserialize, Serialize};
use mode::ProxyModeOptions;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyOptions {
    pub proxy_mode: &'static str,
    pub proxy_pac_url: &'static str,
    pub proxy_server: &'static str,
    pub proxy_bypass_list: &'static str,
}

impl ProxyOptions {
    pub fn new(
        mode: ProxyModeOptions,
        proxy_pac_url: &'static str,
        proxy_server: &'static str,
        proxy_bypass_list: &'static str ) -> Self 
    {        
        let pmode: &'static str;
        let pac: &'static str;
        let pserver: &'static str;
        let pbypass: &'static str;

        match mode {
            ProxyModeOptions::Direct => {
                pmode = "direct";
            },
            ProxyModeOptions::System => {
                pmode = "system";
            },
            ProxyModeOptions::AutoDetect => {
                pmode = "auto_detect";
            },
            ProxyModeOptions::FixedServers => {
                pmode = "fixed_servers";
            },
            ProxyModeOptions::PacUrl => {
                pmode = "pac_url";
            },
        };

        match mode {
            ProxyModeOptions::Direct
            | ProxyModeOptions::System
            | ProxyModeOptions::AutoDetect => {
                pac = "";
                pserver = "";
                pbypass = "";
            },
            ProxyModeOptions::FixedServers => {
                pac = "";
                pserver = proxy_server;
                pbypass = proxy_bypass_list;
            },
            ProxyModeOptions::PacUrl => {
                pac = proxy_pac_url;
                pserver = "";
                pbypass = "";
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