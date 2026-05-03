#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat photo was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPhoto {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
}
