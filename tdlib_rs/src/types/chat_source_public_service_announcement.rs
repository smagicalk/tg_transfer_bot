#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat contains a public service announcement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatSourcePublicServiceAnnouncement {
    /// The type of the announcement
    pub r#type: String,
    /// The text of the announcement
    pub text: String,
}
