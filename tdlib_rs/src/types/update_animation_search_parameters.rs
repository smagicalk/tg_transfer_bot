#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The parameters of animation search through getOption("animation_search_bot_username") bot has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAnimationSearchParameters {
    /// Name of the animation search provider
    pub provider: String,
    /// The new list of emojis suggested for searching
    pub emojis: Vec<String>,
}
