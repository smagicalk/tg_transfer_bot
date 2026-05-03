#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about subscription plan that must be paid by the user to use a chat invite link
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkSubscriptionInfo {
    /// Information about subscription plan that must be paid by the user to use the link
    pub pricing: crate::types::StarSubscriptionPricing,
    /// True, if the user has already paid for the subscription and can use joinChatByInviteLink to join the subscribed chat again
    pub can_reuse: bool,
    /// Identifier of the payment form to use for subscription payment; 0 if the subscription can't be paid
    #[serde_as(as = "DisplayFromStr")]
    pub form_id: i64,
}
