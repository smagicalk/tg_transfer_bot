#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushReceiverId {
    /// The globally unique identifier of push notification subscription
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
}
