#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InviteGroupCallParticipantResult {
    /// The user can't be invited due to their privacy settings
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserPrivacyRestricted", deserialize = "inviteGroupCallParticipantResultUserPrivacyRestricted"))]
    UserPrivacyRestricted,
    /// The user can't be invited because they are already a participant of the call
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserAlreadyParticipant", deserialize = "inviteGroupCallParticipantResultUserAlreadyParticipant"))]
    UserAlreadyParticipant,
    /// The user can't be invited because they were banned by the owner of the call and can be invited back only by the owner of the group call
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserWasBanned", deserialize = "inviteGroupCallParticipantResultUserWasBanned"))]
    UserWasBanned,
    /// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
    #[serde(rename(serialize = "inviteGroupCallParticipantResultSuccess", deserialize = "inviteGroupCallParticipantResultSuccess"))]
    Success(crate::types::InviteGroupCallParticipantResultSuccess),
}
