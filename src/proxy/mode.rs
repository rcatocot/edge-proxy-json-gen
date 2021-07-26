//!
//! # Proxy Modes
//! 
//! Parent: [[`crate::proxy`](crate::proxy)
//! 
//! Defines Proxy Mode types as an enum.
//! 

use serde::{Deserialize, Serialize};

///
/// Enum for defining Proxy Mode types.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyModeOptions {
    /// Direct Connection.
    /// 
    /// No Proxy used.
    /// 
    /// All other [`ProxyOptions`](crate::proxy::ProxyOptions) fields ignored.
    Direct,
    /// System Proxy used.
    /// 
    /// All other [`ProxyOptions`](crate::proxy::ProxyOptions) fields ignored.
    System,
    /// Auto-detet Proxy Settings.
    /// 
    /// All other [`ProxyOptions`](crate::proxy::ProxyOptions) fields ignored.
    AutoDetect,
    /// A Proxy URL will be used.
    /// 
    /// The [`ProxyOptions`](crate::proxy::ProxyOptions) fields, [`proxy_server`](crate::proxy::ProxyOptions::proxy_server)
    /// and [`proxy_bypass_list`](crate::proxy::ProxyOptions::proxy_bypass_list) are used.
    /// 
    /// All other [`ProxyOptions`](crate::proxy::ProxyOptions) fields ignored.
    FixedServers,
    /// Configuration, .pac, file to be used.
    /// 
    /// The [`proxy_pac_url`](crate::proxy::ProxyOptions::proxy_pac_url) field is used.
    /// 
    /// All other [`ProxyOptions`](crate::proxy::ProxyOptions) fields ignored.
    PacUrl,
}