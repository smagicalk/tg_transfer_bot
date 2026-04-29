#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is uploading a video note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionUploadingVideoNote {
    /// Upload progress, as a percentage
    pub progress: i32,
}
