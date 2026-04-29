#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceLink {
    /// The referrer from the link
    pub referrer: String,
}
