#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUpgradedGift {
    /// Name of the unique gift
    pub name: String,
}
