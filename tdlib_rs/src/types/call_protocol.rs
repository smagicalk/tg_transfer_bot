#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Specifies the supported call protocols
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProtocol {
    /// True, if UDP peer-to-peer connections are supported
    pub udp_p2p: bool,
    /// True, if connection through UDP reflectors is supported
    pub udp_reflector: bool,
    /// The minimum supported API layer; use 65
    pub min_layer: i32,
    /// The maximum supported API layer; use 92
    pub max_layer: i32,
    /// List of supported tgcalls versions
    pub library_versions: Vec<String>,
}
