#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportAuthorizationForm {
    /// Contains information about a Telegram Passport authorization form that was requested
    #[serde(rename(
        serialize = "passportAuthorizationForm",
        deserialize = "passportAuthorizationForm"
    ))]
    PassportAuthorizationForm(crate::types::PassportAuthorizationForm),
}
