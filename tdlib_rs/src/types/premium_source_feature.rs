#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A user tried to use a Premium feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceFeature {
    /// The used feature
    pub feature: crate::enums::PremiumFeature,
}
