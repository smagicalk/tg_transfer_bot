#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a business chat link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLink {
    /// The HTTPS link
    pub link: String,
    /// Message draft text that will be added to the input field
    pub text: crate::types::FormattedText,
    /// Link title
    pub title: String,
    /// Number of times the link was used
    pub view_count: i32,
}
