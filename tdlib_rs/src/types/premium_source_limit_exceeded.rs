#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A limit was exceeded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceLimitExceeded {
    /// Type of the exceeded limit
    pub limit_type: crate::enums::PremiumLimitType,
}
