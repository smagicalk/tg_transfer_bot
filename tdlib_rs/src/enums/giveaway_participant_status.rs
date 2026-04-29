#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayParticipantStatus {
    /// The user is eligible for the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusEligible", deserialize = "giveawayParticipantStatusEligible"))]
    Eligible,
    /// The user participates in the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusParticipating", deserialize = "giveawayParticipantStatusParticipating"))]
    Participating,
    /// The user can't participate in the giveaway, because they have already been member of the chat
    #[serde(rename(serialize = "giveawayParticipantStatusAlreadyWasMember", deserialize = "giveawayParticipantStatusAlreadyWasMember"))]
    AlreadyWasMember(crate::types::GiveawayParticipantStatusAlreadyWasMember),
    /// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusAdministrator", deserialize = "giveawayParticipantStatusAdministrator"))]
    Administrator(crate::types::GiveawayParticipantStatusAdministrator),
    /// The user can't participate in the giveaway, because they phone number is from a disallowed country
    #[serde(rename(serialize = "giveawayParticipantStatusDisallowedCountry", deserialize = "giveawayParticipantStatusDisallowedCountry"))]
    DisallowedCountry(crate::types::GiveawayParticipantStatusDisallowedCountry),
}
