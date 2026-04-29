#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUnknownDeepLink {
    /// Link to be passed to getDeepLinkInfo
    pub link: String,
}
