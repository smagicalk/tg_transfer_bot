#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an ongoing giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiveawayInfoOngoing {
    /// Point in time (Unix timestamp) when the giveaway was created
    pub creation_date: i32,
    /// Status of the current user in the giveaway
    pub status: crate::enums::GiveawayParticipantStatus,
    /// True, if the giveaway has ended and results are being prepared
    pub is_ended: bool,
}
