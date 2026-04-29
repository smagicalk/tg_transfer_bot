#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a subscription in a bot or a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionTypeBot {
    /// True, if the subscription was canceled by the bot and can't be extended
    pub is_canceled_by_bot: bool,
    /// Subscription invoice title
    pub title: String,
    /// Subscription invoice photo
    pub photo: crate::types::Photo,
    /// The link to the subscription invoice
    pub invoice_link: String,
}
