#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A HTTP transparent proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeHttp {
    /// Username for logging in; may be empty
    pub username: String,
    /// Password for logging in; may be empty
    pub password: String,
    /// Pass true if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method
    pub http_only: bool,
}
