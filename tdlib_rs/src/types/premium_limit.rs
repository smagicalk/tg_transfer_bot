#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a limit, increased for Premium users
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimit {
    /// The type of the limit
    pub r#type: crate::enums::PremiumLimitType,
    /// Default value of the limit
    pub default_value: i32,
    /// Value of the limit for Premium users
    pub premium_value: i32,
}
