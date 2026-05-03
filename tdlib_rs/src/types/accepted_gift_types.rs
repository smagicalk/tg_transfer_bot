#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes gift types that are accepted by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AcceptedGiftTypes {
    /// True, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,
    /// True, if limited regular gifts are accepted
    pub limited_gifts: bool,
    /// True, if upgraded gifts and regular gifts that can be upgraded for free are accepted
    pub upgraded_gifts: bool,
    /// True, if gifts from channels are accepted subject to other restrictions
    pub gifts_from_channels: bool,
    /// True, if Telegram Premium subscription is accepted
    pub premium_subscription: bool,
}
