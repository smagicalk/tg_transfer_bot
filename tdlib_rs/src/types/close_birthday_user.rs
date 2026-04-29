#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a user who had or will have a birthday soon
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CloseBirthdayUser {
    /// User identifier
    pub user_id: i64,
    /// Birthdate of the user
    pub birthdate: crate::types::Birthdate,
}
