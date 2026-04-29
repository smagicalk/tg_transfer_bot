#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserLink {
    /// Contains an HTTPS URL, which can be used to get information about a user
    #[serde(rename(serialize = "userLink", deserialize = "userLink"))]
    UserLink(crate::types::UserLink),
}
