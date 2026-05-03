#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains custom information about the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserSupportInfo {
    /// Information message
    pub message: crate::types::FormattedText,
    /// Information author
    pub author: String,
    /// Information change date
    pub date: i32,
}
