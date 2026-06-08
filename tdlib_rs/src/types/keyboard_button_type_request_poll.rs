#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A button that allows the user to create and send a poll when pressed; available only in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestPoll {
    /// If true, only regular polls must be allowed to create
    pub force_regular: bool,
    /// If true, only polls in quiz mode must be allowed to create
    pub force_quiz: bool,
}
