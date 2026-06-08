#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains an HTTPS URL, which can be used to get information about a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserLink {
    /// The URL
    pub url: String,
    /// Left time for which the link is valid, in seconds; 0 if the link is a public username link
    pub expires_in: i32,
}
