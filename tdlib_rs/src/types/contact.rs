#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a contact of a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Contact {
    /// Phone number of the user
    pub phone_number: String,
    /// First name of the user; 1-64 characters
    pub first_name: String,
    /// Last name of the user; 0-64 characters
    pub last_name: String,
    /// Additional data about the user in a form of vCard; 0-2048 bytes in length
    pub vcard: String,
    /// Identifier of the user, if known; 0 otherwise
    pub user_id: i64,
}
