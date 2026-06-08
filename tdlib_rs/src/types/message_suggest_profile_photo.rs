#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A profile photo was suggested to a user in a private chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestProfilePhoto {
    /// The suggested chat photo. Use the method setProfilePhoto with inputChatPhotoPrevious to apply the photo
    pub photo: crate::types::ChatPhoto,
}
