#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is uploading a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionUploadingVoiceNote {
    /// Upload progress, as a percentage
    pub progress: i32,
}
