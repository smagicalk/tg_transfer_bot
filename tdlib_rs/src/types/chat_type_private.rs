#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An ordinary chat with a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypePrivate {
    /// User identifier
    pub user_id: i64,
}
