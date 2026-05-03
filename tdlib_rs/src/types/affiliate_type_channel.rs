#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The affiliate is a channel chat where the current user has can_post_messages administrator right
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateTypeChannel {
    /// Identifier of the channel chat
    pub chat_id: i64,
}
