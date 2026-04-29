#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A user tried to use a Premium story feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceStoryFeature {
    /// The used feature
    pub feature: crate::enums::PremiumStoryFeature,
}
