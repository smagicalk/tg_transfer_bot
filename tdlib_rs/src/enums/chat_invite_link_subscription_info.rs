#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkSubscriptionInfo {
    /// Contains information about subscription plan that must be paid by the user to use a chat invite link
    #[serde(rename(serialize = "chatInviteLinkSubscriptionInfo", deserialize = "chatInviteLinkSubscriptionInfo"))]
    ChatInviteLinkSubscriptionInfo(crate::types::ChatInviteLinkSubscriptionInfo),
}
