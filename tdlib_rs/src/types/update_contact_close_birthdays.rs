#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of contacts that had birthdays recently or will have birthday soon has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateContactCloseBirthdays {
    /// List of contact users with close birthday
    pub close_birthday_users: Vec<crate::types::CloseBirthdayUser>,
}
