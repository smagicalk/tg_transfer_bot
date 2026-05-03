#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a Telegram Business account
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessInfo {
    /// Location of the business; may be null if none
    pub location: Option<crate::types::BusinessLocation>,
    /// Opening hours of the business; may be null if none. The hours are guaranteed to be valid and has already been split by week days
    pub opening_hours: Option<crate::types::BusinessOpeningHours>,
    /// Opening hours of the business in the local time; may be null if none. The hours are guaranteed to be valid and has already been split by week days.
    /// Local time zone identifier will be empty. An updateUserFullInfo update is not triggered when value of this field changes
    pub local_opening_hours: Option<crate::types::BusinessOpeningHours>,
    /// Time left before the business will open the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
    pub next_open_in: i32,
    /// Time left before the business will close the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
    pub next_close_in: i32,
    /// The greeting message; may be null if none or the Business account is not of the current user
    pub greeting_message_settings: Option<crate::types::BusinessGreetingMessageSettings>,
    /// The away message; may be null if none or the Business account is not of the current user
    pub away_message_settings: Option<crate::types::BusinessAwayMessageSettings>,
    /// Information about start page of the account; may be null if none
    pub start_page: Option<crate::types::BusinessStartPage>,
}
