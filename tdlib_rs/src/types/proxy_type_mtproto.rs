#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An MTProto proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeMtproto {
    /// The proxy's secret in hexadecimal encoding
    pub secret: String,
}
