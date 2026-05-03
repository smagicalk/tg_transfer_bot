#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUser {
    /// New data about the user
    pub user: crate::types::User,
}
