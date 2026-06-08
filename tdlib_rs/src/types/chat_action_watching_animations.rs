#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user is watching animations sent by the other party by clicking on an animated emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionWatchingAnimations {
    /// The animated emoji
    pub emoji: String,
}
