#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A SOCKS5 proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeSocks5 {
    /// Username for logging in; may be empty
    pub username: String,
    /// Password for logging in; may be empty
    pub password: String,
}
