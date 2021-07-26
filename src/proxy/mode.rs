use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyModeOptions {
    Direct,
    System,
    AutoDetect,
    FixedServers,
    PacUrl,
}