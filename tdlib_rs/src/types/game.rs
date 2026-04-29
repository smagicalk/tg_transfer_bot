#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a game. Use getInternalLink with internalLinkTypeGame to share the game
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Game {
    /// Unique game identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Game short name
    pub short_name: String,
    /// Game title
    pub title: String,
    /// Game text, usually containing scoreboards for a game
    pub text: crate::types::FormattedText,
    /// Game description
    pub description: String,
    /// Game photo
    pub photo: crate::types::Photo,
    /// Game animation; may be null
    pub animation: Option<crate::types::Animation>,
}
