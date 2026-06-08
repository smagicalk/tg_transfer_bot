#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains Telegram terms of service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TermsOfService {
    /// Text of the terms of service
    pub text: crate::types::FormattedText,
    /// The minimum age of a user to be able to accept the terms; 0 if age isn't restricted
    pub min_user_age: i32,
    /// True, if a blocking popup with terms of service must be shown to the user
    pub show_popup: bool,
}
