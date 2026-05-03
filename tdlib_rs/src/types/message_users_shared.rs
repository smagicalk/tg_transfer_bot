#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The current user shared users, which were requested by the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageUsersShared {
    /// The shared users
    pub users: Vec<crate::types::SharedUser>,
    /// Identifier of the keyboard button with the request
    pub button_id: i32,
}
