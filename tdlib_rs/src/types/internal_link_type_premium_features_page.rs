#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePremiumFeaturesPage {
    /// Referrer specified in the link
    pub referrer: String,
}
