#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains basic information about another user who started a chat with the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccountInfo {
    /// Month when the user was registered in Telegram; 0-12; may be 0 if unknown
    pub registration_month: i32,
    /// Year when the user was registered in Telegram; 0-9999; may be 0 if unknown
    pub registration_year: i32,
    /// A two-letter ISO 3166-1 alpha-2 country code based on the phone number of the user; may be empty if unknown
    pub phone_number_country_code: String,
    /// Point in time (Unix timestamp) when the user changed name last time; 0 if unknown
    pub last_name_change_date: i32,
    /// Point in time (Unix timestamp) when the user changed photo last time; 0 if unknown
    pub last_photo_change_date: i32,
}
