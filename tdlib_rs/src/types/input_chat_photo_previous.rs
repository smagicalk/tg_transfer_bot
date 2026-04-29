#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A previously used profile photo of the current user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChatPhotoPrevious {
    /// Identifier of the current user's profile photo to reuse
    #[serde_as(as = "DisplayFromStr")]
    pub chat_photo_id: i64,
}
