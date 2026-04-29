#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a Telegram Passport authorization form that was requested
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportAuthorizationForm {
    /// Unique identifier of the authorization form
    pub id: i32,
    /// Telegram Passport elements that must be provided to complete the form
    pub required_elements: Vec<crate::types::PassportRequiredElement>,
    /// URL for the privacy policy of the service; may be empty
    pub privacy_policy_url: String,
}
