//!
//! TDLib `poll` domain enums.
//!
//! Types, enums, and functions for polls, quiz questions, and voter answers.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `PollOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollOption {
    /// Describes one answer option of a poll
    #[serde(rename(serialize = "pollOption", deserialize = "pollOption"))]
    PollOption(Box<crate::types::PollOption>),
}

impl PollOption {
    /// Convenience constructor to create a [`PollOption::PollOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_option(val: crate::types::PollOption) -> Self {
        Self::PollOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PollOption`] into [`PollOption`].
impl From<crate::types::PollOption> for PollOption {
    fn from(val: crate::types::PollOption) -> Self {
        Self::PollOption(Box::new(val))
    }
}

/// TDLib `InputPollOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPollOption {
    /// Describes one answer option of a poll to be created
    #[serde(rename(serialize = "inputPollOption", deserialize = "inputPollOption"))]
    InputPollOption(Box<crate::types::InputPollOption>),
}

impl InputPollOption {
    /// Convenience constructor to create a [`InputPollOption::InputPollOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_poll_option(val: crate::types::InputPollOption) -> Self {
        Self::InputPollOption(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPollOption`] into [`InputPollOption`].
impl From<crate::types::InputPollOption> for InputPollOption {
    fn from(val: crate::types::InputPollOption) -> Self {
        Self::InputPollOption(Box::new(val))
    }
}

/// Describes the type of poll
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollType {
    /// A regular poll
    #[serde(rename(serialize = "pollTypeRegular", deserialize = "pollTypeRegular"))]
    Regular,
    /// A poll in quiz mode, which has predefined correct answers
    #[serde(rename(serialize = "pollTypeQuiz", deserialize = "pollTypeQuiz"))]
    Quiz(Box<crate::types::PollTypeQuiz>),
}

impl PollType {
    /// Convenience constructor to create a [`PollType::Quiz`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quiz(val: crate::types::PollTypeQuiz) -> Self {
        Self::Quiz(Box::new(val))
    }

}

/// Converts a [`crate::types::PollTypeQuiz`] into [`PollType`].
impl From<crate::types::PollTypeQuiz> for PollType {
    fn from(val: crate::types::PollTypeQuiz) -> Self {
        Self::Quiz(Box::new(val))
    }
}

/// Describes the type of poll to send
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPollType {
    /// A regular poll
    #[serde(rename(serialize = "inputPollTypeRegular", deserialize = "inputPollTypeRegular"))]
    Regular(Box<crate::types::InputPollTypeRegular>),
    /// A poll in quiz mode, which has predefined correct answers
    #[serde(rename(serialize = "inputPollTypeQuiz", deserialize = "inputPollTypeQuiz"))]
    Quiz(Box<crate::types::InputPollTypeQuiz>),
}

impl InputPollType {
    /// Convenience constructor to create a [`InputPollType::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::InputPollTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollType::Quiz`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quiz(val: crate::types::InputPollTypeQuiz) -> Self {
        Self::Quiz(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPollTypeRegular`] into [`InputPollType`].
impl From<crate::types::InputPollTypeRegular> for InputPollType {
    fn from(val: crate::types::InputPollTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollTypeQuiz`] into [`InputPollType`].
impl From<crate::types::InputPollTypeQuiz> for InputPollType {
    fn from(val: crate::types::InputPollTypeQuiz) -> Self {
        Self::Quiz(Box::new(val))
    }
}

/// Reason of vote restriction in the poll for the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoteRestrictionReason {
    /// The poll is closed
    #[serde(rename(serialize = "pollVoteRestrictionReasonClosed", deserialize = "pollVoteRestrictionReasonClosed"))]
    Closed,
    /// The poll isn't sent yet
    #[serde(rename(serialize = "pollVoteRestrictionReasonYetUnsent", deserialize = "pollVoteRestrictionReasonYetUnsent"))]
    YetUnsent,
    /// The poll is from a scheduled message
    #[serde(rename(serialize = "pollVoteRestrictionReasonScheduled", deserialize = "pollVoteRestrictionReasonScheduled"))]
    Scheduled,
    /// The user is from a country, users from which aren't allowed to vote
    #[serde(rename(serialize = "pollVoteRestrictionReasonCountryRestricted", deserialize = "pollVoteRestrictionReasonCountryRestricted"))]
    CountryRestricted(Box<crate::types::PollVoteRestrictionReasonCountryRestricted>),
    /// The user must be a member of the chat for at least a day to vote
    #[serde(rename(serialize = "pollVoteRestrictionReasonMembershipRequired", deserialize = "pollVoteRestrictionReasonMembershipRequired"))]
    MembershipRequired(Box<crate::types::PollVoteRestrictionReasonMembershipRequired>),
    /// The poll can't be voted by the user due to some other reason
    #[serde(rename(serialize = "pollVoteRestrictionReasonOther", deserialize = "pollVoteRestrictionReasonOther"))]
    Other,
}

impl PollVoteRestrictionReason {
    /// Convenience constructor to create a [`PollVoteRestrictionReason::CountryRestricted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn country_restricted(val: crate::types::PollVoteRestrictionReasonCountryRestricted) -> Self {
        Self::CountryRestricted(Box::new(val))
    }

    /// Convenience constructor to create a [`PollVoteRestrictionReason::MembershipRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn membership_required(val: crate::types::PollVoteRestrictionReasonMembershipRequired) -> Self {
        Self::MembershipRequired(Box::new(val))
    }

}

/// Converts a [`crate::types::PollVoteRestrictionReasonCountryRestricted`] into [`PollVoteRestrictionReason`].
impl From<crate::types::PollVoteRestrictionReasonCountryRestricted> for PollVoteRestrictionReason {
    fn from(val: crate::types::PollVoteRestrictionReasonCountryRestricted) -> Self {
        Self::CountryRestricted(Box::new(val))
    }
}

/// Converts a [`crate::types::PollVoteRestrictionReasonMembershipRequired`] into [`PollVoteRestrictionReason`].
impl From<crate::types::PollVoteRestrictionReasonMembershipRequired> for PollVoteRestrictionReason {
    fn from(val: crate::types::PollVoteRestrictionReasonMembershipRequired) -> Self {
        Self::MembershipRequired(Box::new(val))
    }
}

/// TDLib `Poll` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Poll {
    /// Describes a poll
    #[serde(rename(serialize = "poll", deserialize = "poll"))]
    Poll(Box<crate::types::Poll>),
}

impl Poll {
    /// Convenience constructor to create a [`Poll::Poll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll(val: crate::types::Poll) -> Self {
        Self::Poll(Box::new(val))
    }

}

/// Converts a [`crate::types::Poll`] into [`Poll`].
impl From<crate::types::Poll> for Poll {
    fn from(val: crate::types::Poll) -> Self {
        Self::Poll(Box::new(val))
    }
}

/// TDLib `PollVoter` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoter {
    /// Represents a poll voter
    #[serde(rename(serialize = "pollVoter", deserialize = "pollVoter"))]
    PollVoter(Box<crate::types::PollVoter>),
}

impl PollVoter {
    /// Convenience constructor to create a [`PollVoter::PollVoter`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_voter(val: crate::types::PollVoter) -> Self {
        Self::PollVoter(Box::new(val))
    }

}

/// Converts a [`crate::types::PollVoter`] into [`PollVoter`].
impl From<crate::types::PollVoter> for PollVoter {
    fn from(val: crate::types::PollVoter) -> Self {
        Self::PollVoter(Box::new(val))
    }
}

/// TDLib `PollVoters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoters {
    /// Represents a list of poll voters
    #[serde(rename(serialize = "pollVoters", deserialize = "pollVoters"))]
    PollVoters(Box<crate::types::PollVoters>),
}

impl PollVoters {
    /// Convenience constructor to create a [`PollVoters::PollVoters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_voters(val: crate::types::PollVoters) -> Self {
        Self::PollVoters(Box::new(val))
    }

}

/// Converts a [`crate::types::PollVoters`] into [`PollVoters`].
impl From<crate::types::PollVoters> for PollVoters {
    fn from(val: crate::types::PollVoters) -> Self {
        Self::PollVoters(Box::new(val))
    }
}

/// Contains the media in a poll
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollMedia {
    /// An animation
    #[serde(rename(serialize = "pollMediaAnimation", deserialize = "pollMediaAnimation"))]
    Animation(Box<crate::types::PollMediaAnimation>),
    /// An audio
    #[serde(rename(serialize = "pollMediaAudio", deserialize = "pollMediaAudio"))]
    Audio(Box<crate::types::PollMediaAudio>),
    /// A document (general file)
    #[serde(rename(serialize = "pollMediaDocument", deserialize = "pollMediaDocument"))]
    Document(Box<crate::types::PollMediaDocument>),
    /// A link
    #[serde(rename(serialize = "pollMediaLink", deserialize = "pollMediaLink"))]
    Link(Box<crate::types::PollMediaLink>),
    /// A location
    #[serde(rename(serialize = "pollMediaLocation", deserialize = "pollMediaLocation"))]
    Location(Box<crate::types::PollMediaLocation>),
    /// A photo
    #[serde(rename(serialize = "pollMediaPhoto", deserialize = "pollMediaPhoto"))]
    Photo(Box<crate::types::PollMediaPhoto>),
    /// A sticker
    #[serde(rename(serialize = "pollMediaSticker", deserialize = "pollMediaSticker"))]
    Sticker(Box<crate::types::PollMediaSticker>),
    /// A venue
    #[serde(rename(serialize = "pollMediaVenue", deserialize = "pollMediaVenue"))]
    Venue(Box<crate::types::PollMediaVenue>),
    /// A video
    #[serde(rename(serialize = "pollMediaVideo", deserialize = "pollMediaVideo"))]
    Video(Box<crate::types::PollMediaVideo>),
}

impl PollMedia {
    /// Convenience constructor to create a [`PollMedia::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::PollMediaAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::PollMediaAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::PollMediaDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::PollMediaLink) -> Self {
        Self::Link(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::PollMediaLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::PollMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::PollMediaSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::PollMediaVenue) -> Self {
        Self::Venue(Box::new(val))
    }

    /// Convenience constructor to create a [`PollMedia::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::PollMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::PollMediaAnimation`] into [`PollMedia`].
impl From<crate::types::PollMediaAnimation> for PollMedia {
    fn from(val: crate::types::PollMediaAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaAudio`] into [`PollMedia`].
impl From<crate::types::PollMediaAudio> for PollMedia {
    fn from(val: crate::types::PollMediaAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaDocument`] into [`PollMedia`].
impl From<crate::types::PollMediaDocument> for PollMedia {
    fn from(val: crate::types::PollMediaDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaLink`] into [`PollMedia`].
impl From<crate::types::PollMediaLink> for PollMedia {
    fn from(val: crate::types::PollMediaLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaLocation`] into [`PollMedia`].
impl From<crate::types::PollMediaLocation> for PollMedia {
    fn from(val: crate::types::PollMediaLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaPhoto`] into [`PollMedia`].
impl From<crate::types::PollMediaPhoto> for PollMedia {
    fn from(val: crate::types::PollMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaSticker`] into [`PollMedia`].
impl From<crate::types::PollMediaSticker> for PollMedia {
    fn from(val: crate::types::PollMediaSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaVenue`] into [`PollMedia`].
impl From<crate::types::PollMediaVenue> for PollMedia {
    fn from(val: crate::types::PollMediaVenue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// Converts a [`crate::types::PollMediaVideo`] into [`PollMedia`].
impl From<crate::types::PollMediaVideo> for PollMedia {
    fn from(val: crate::types::PollMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// The content of a poll media to send
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPollMedia {
    /// An animation
    #[serde(rename(serialize = "inputPollMediaAnimation", deserialize = "inputPollMediaAnimation"))]
    Animation(Box<crate::types::InputPollMediaAnimation>),
    /// An audio
    #[serde(rename(serialize = "inputPollMediaAudio", deserialize = "inputPollMediaAudio"))]
    Audio(Box<crate::types::InputPollMediaAudio>),
    /// A document (general file)
    #[serde(rename(serialize = "inputPollMediaDocument", deserialize = "inputPollMediaDocument"))]
    Document(Box<crate::types::InputPollMediaDocument>),
    /// A link
    #[serde(rename(serialize = "inputPollMediaLink", deserialize = "inputPollMediaLink"))]
    Link(Box<crate::types::InputPollMediaLink>),
    /// A location
    #[serde(rename(serialize = "inputPollMediaLocation", deserialize = "inputPollMediaLocation"))]
    Location(Box<crate::types::InputPollMediaLocation>),
    /// A photo
    #[serde(rename(serialize = "inputPollMediaPhoto", deserialize = "inputPollMediaPhoto"))]
    Photo(Box<crate::types::InputPollMediaPhoto>),
    /// A sticker
    #[serde(rename(serialize = "inputPollMediaSticker", deserialize = "inputPollMediaSticker"))]
    Sticker(Box<crate::types::InputPollMediaSticker>),
    /// A venue
    #[serde(rename(serialize = "inputPollMediaVenue", deserialize = "inputPollMediaVenue"))]
    Venue(Box<crate::types::InputPollMediaVenue>),
    /// A video
    #[serde(rename(serialize = "inputPollMediaVideo", deserialize = "inputPollMediaVideo"))]
    Video(Box<crate::types::InputPollMediaVideo>),
}

impl InputPollMedia {
    /// Convenience constructor to create a [`InputPollMedia::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::InputPollMediaAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::InputPollMediaAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::InputPollMediaDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::InputPollMediaLink) -> Self {
        Self::Link(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::InputPollMediaLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InputPollMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::InputPollMediaSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::InputPollMediaVenue) -> Self {
        Self::Venue(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPollMedia::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InputPollMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPollMediaAnimation`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaAnimation> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaAudio`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaAudio> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaDocument`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaDocument> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaLink`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaLink> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaLocation`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaLocation> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaPhoto`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaPhoto> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaSticker`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaSticker> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaVenue`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaVenue> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaVenue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPollMediaVideo`] into [`InputPollMedia`].
impl From<crate::types::InputPollMediaVideo> for InputPollMedia {
    fn from(val: crate::types::InputPollMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// TDLib `PollOptionProperties` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollOptionProperties {
    /// Contains properties of a poll option and describes actions that can be done with the option right now
    #[serde(rename(serialize = "pollOptionProperties", deserialize = "pollOptionProperties"))]
    PollOptionProperties(Box<crate::types::PollOptionProperties>),
}

impl PollOptionProperties {
    /// Convenience constructor to create a [`PollOptionProperties::PollOptionProperties`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_option_properties(val: crate::types::PollOptionProperties) -> Self {
        Self::PollOptionProperties(Box::new(val))
    }

}

/// Converts a [`crate::types::PollOptionProperties`] into [`PollOptionProperties`].
impl From<crate::types::PollOptionProperties> for PollOptionProperties {
    fn from(val: crate::types::PollOptionProperties) -> Self {
        Self::PollOptionProperties(Box::new(val))
    }
}

/// TDLib `PollVoteStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoteStatistics {
    /// A detailed statistics about poll votes
    #[serde(rename(serialize = "pollVoteStatistics", deserialize = "pollVoteStatistics"))]
    PollVoteStatistics(Box<crate::types::PollVoteStatistics>),
}

impl PollVoteStatistics {
    /// Convenience constructor to create a [`PollVoteStatistics::PollVoteStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_vote_statistics(val: crate::types::PollVoteStatistics) -> Self {
        Self::PollVoteStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::PollVoteStatistics`] into [`PollVoteStatistics`].
impl From<crate::types::PollVoteStatistics> for PollVoteStatistics {
    fn from(val: crate::types::PollVoteStatistics) -> Self {
        Self::PollVoteStatistics(Box::new(val))
    }
}

