#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about features, available to Business user accounts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatures {
    /// The list of available business features
    pub features: Vec<crate::enums::BusinessFeature>,
}
