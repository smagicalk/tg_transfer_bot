#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionUnavailabilityReason {
    /// The user is an anonymous administrator in the supergroup, but isn't a creator of it, so they can't vote on behalf of the supergroup
    #[serde(rename(
        serialize = "reactionUnavailabilityReasonAnonymousAdministrator",
        deserialize = "reactionUnavailabilityReasonAnonymousAdministrator"
    ))]
    AnonymousAdministrator,
    /// The user isn't a member of the supergroup and can't send messages and reactions there without joining
    #[serde(rename(
        serialize = "reactionUnavailabilityReasonGuest",
        deserialize = "reactionUnavailabilityReasonGuest"
    ))]
    Guest,
}
