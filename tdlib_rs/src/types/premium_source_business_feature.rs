#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A user tried to use a Business feature
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceBusinessFeature {
    /// The used feature; pass null if none specific feature was used
    pub feature: Option<crate::enums::BusinessFeature>,
}
