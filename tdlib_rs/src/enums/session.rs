#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Session {
    /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
    #[serde(rename(serialize = "session", deserialize = "session"))]
    Session(crate::types::Session),
}
