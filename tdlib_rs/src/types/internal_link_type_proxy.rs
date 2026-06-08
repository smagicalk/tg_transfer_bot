#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeProxy {
    /// The proxy; may be null if the proxy is unsupported, in which case an alert can be shown to the user
    pub proxy: Option<crate::types::Proxy>,
}
