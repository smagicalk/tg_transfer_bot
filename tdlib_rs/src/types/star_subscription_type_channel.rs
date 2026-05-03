#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a subscription to a channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionTypeChannel {
    /// True, if the subscription is active and the user can use the method reuseStarSubscription to join the subscribed chat again
    pub can_reuse: bool,
    /// The invite link that can be used to renew the subscription if it has been expired; may be empty, if the link isn't available anymore
    pub invite_link: String,
}
