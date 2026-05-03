#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A newly created supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSupergroupChatCreate {
    /// Title of the supergroup or channel
    pub title: String,
}
