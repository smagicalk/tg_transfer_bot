//!
//! TDLib `poll` domain types.
//!
//! Types, enums, and functions for polls, quiz questions, and voter answers.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes one answer option of a poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollOption {
    /// Unique identifier of the option in the poll; may be empty if yet unassigned
    pub id: String,
    /// Option text; 1-100 characters; may contain only custom emoji entities
    pub text: crate::types::FormattedText,
    /// Option media; may be null if none. If present, currently, can be only of the types pollMediaAnimation, pollMediaLink, pollMediaLocation, pollMediaPhoto, pollMediaSticker, pollMediaVenue, or pollMediaVideo
    pub media: Option<crate::enums::PollMedia>,
    /// Number of voters for this option, available only for closed or voted polls, or if the current user is the creator of the poll
    pub voter_count: i32,
    /// The percentage of votes for this option; 0-100
    pub vote_percentage: i32,
    /// Identifiers of recent voters for the option, if the poll is non-anonymous and poll results are available
    pub recent_voter_ids: Vec<crate::enums::MessageSender>,
    /// True, if the option was chosen by the user
    pub is_chosen: bool,
    /// True, if the option is being chosen by a pending setPollAnswer request
    pub is_being_chosen: bool,
    /// Identifier of the user or chat who added the option; may be null if the option existed from creation of the poll
    pub author: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the option was added; 0 if the option existed from creation of the poll
    pub addition_date: i32,
}

/// Describes one answer option of a poll to be created
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollOption {
    /// Option text; 1-100 characters. Only custom emoji entities are allowed to be added and only by Premium users
    pub text: crate::types::FormattedText,
    /// Option media; pass null if none. Must be one of the following types:
    /// inputPollMediaAnimation, inputPollMediaLink, inputPollMediaLocation, inputPollMediaPhoto, inputPollMediaSticker, inputPollMediaVenue, or inputPollMediaVideo without caption
    pub media: Option<crate::enums::InputPollMedia>,
}

/// A regular poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollTypeRegular {
}

/// A poll in quiz mode, which has predefined correct answers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollTypeQuiz {
    /// Increasing list of 0-based identifiers of the correct answer options; empty for a yet unanswered poll
    pub correct_option_ids: Vec<i32>,
    /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon; empty for a yet unanswered poll
    pub explanation: crate::types::FormattedText,
    /// Media that is shown when the user chooses an incorrect answer or taps on the lamp icon; may be null if none or the poll is unanswered yet.
    /// If present, currently, can be only of the types pollMediaAnimation, pollMediaAudio, pollMediaDocument, pollMediaLocation, pollMediaPhoto, pollMediaVenue, or pollMediaVideo
    pub explanation_media: Option<crate::enums::PollMedia>,
}

/// A regular poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollTypeRegular {
    /// True, if answer options can be added to the poll after creation; not supported in channel chats and for anonymous polls
    pub allow_adding_options: bool,
}

/// A poll in quiz mode, which has predefined correct answers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollTypeQuiz {
    /// Increasing list of 0-based identifiers of the correct answer options; must be non-empty
    pub correct_option_ids: Vec<i32>,
    /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon; 0-200 characters with at most 2 line feeds
    pub explanation: crate::types::FormattedText,
    /// Media that is shown when the user chooses an incorrect answer or taps on the lamp icon; pass null if none. Must be one of the following types:
    /// inputPollMediaAnimation, inputPollMediaAudio, inputPollMediaDocument, inputPollMediaLocation, inputPollMediaPhoto, inputPollMediaVenue, or inputPollMediaVideo without caption
    pub explanation_media: Option<crate::enums::InputPollMedia>,
}

/// The poll is closed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonClosed {
}

/// The poll isn't sent yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonYetUnsent {
}

/// The poll is from a scheduled message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonScheduled {
}

/// The user is from a country, users from which aren't allowed to vote
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonCountryRestricted {
    /// Two-letter ISO 3166-1 alpha-2 code of the current user's country
    pub country_code: String,
}

/// The user must be a member of the chat for at least a day to vote
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonMembershipRequired {
    /// Identifier of the chat which must be joined for at least a day before the user can vote
    pub chat_id: i64,
}

/// The poll can't be voted by the user due to some other reason
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoteRestrictionReasonOther {
}

/// Describes a poll
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Poll {
    /// Unique poll identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Poll question; 1-300 characters; may contain only custom emoji entities
    pub question: crate::types::FormattedText,
    /// List of poll answer options
    pub options: Vec<crate::types::PollOption>,
    /// Total number of voters, participating in the poll
    pub total_voter_count: i32,
    /// Identifiers of recent voters, if the poll is non-anonymous and poll results are available
    pub recent_voter_ids: Vec<crate::enums::MessageSender>,
    /// True, if the current user can get voters in the poll using getPollVoters
    pub can_get_voters: bool,
    /// True, if the current user can see results of the poll
    pub can_see_results: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// True, if multiple answer options can be chosen simultaneously
    pub allows_multiple_answers: bool,
    /// True, if the poll can be answered multiple times
    pub allows_revoting: bool,
    /// True, if only the users that are members of the chat for more than a day will be able to vote
    pub members_only: bool,
    /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be able to vote. If empty, then all users can participate in the poll
    pub country_codes: Vec<String>,
    /// The list of 0-based poll identifiers in which the options of the poll must be shown; empty if the order of options must not be changed
    pub option_order: Vec<i32>,
    /// Type of the poll
    #[serde(rename = "type")]
    pub r#type: crate::enums::PollType,
    /// Amount of time the poll will be active after creation, in seconds
    pub open_period: i32,
    /// Point in time (Unix timestamp) when the poll will automatically be closed
    pub close_date: i32,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// The reason describing, why the current user can't vote in the poll; may be null if the user can vote in the poll
    pub vote_restriction_reason: Option<crate::enums::PollVoteRestrictionReason>,
}

/// Represents a poll voter
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollVoter {
    /// The voter identifier
    pub voter_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the vote was added
    pub date: i32,
}

/// Represents a list of poll voters
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoters {
    /// Approximate total number of poll voters found
    pub total_count: i32,
    /// List of poll voters
    pub voters: Vec<crate::types::PollVoter>,
}

/// A button that allows the user to create and send a poll when pressed; available only in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestPoll {
    /// If true, only regular polls must be allowed to create
    pub force_regular: bool,
    /// If true, only polls in quiz mode must be allowed to create
    pub force_quiz: bool,
}

/// An animation
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaAnimation {
    /// The animation
    pub animation: crate::types::Animation,
}

/// An audio
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaAudio {
    /// The audio
    pub audio: crate::types::Audio,
}

/// A document (general file)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaDocument {
    /// The document
    pub document: crate::types::Document,
}

/// A link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaLink {
    /// URL of the link
    pub url: String,
    /// Preview of the link; may be null if unknown
    pub link_preview: Option<crate::types::LinkPreview>,
}

/// A location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollMediaLocation {
    /// The location
    pub location: crate::types::Location,
}

/// A photo
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaPhoto {
    /// The photo
    pub photo: crate::types::Photo,
    /// The video representing the live photo; may be null if the photo is static
    pub video: Option<crate::types::Video>,
}

/// A venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollMediaVenue {
    /// The venue
    pub venue: crate::types::Venue,
}

/// A video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaVideo {
    /// The video description
    pub video: crate::types::Video,
    /// Alternative qualities of the video
    pub alternative_videos: Vec<crate::types::AlternativeVideo>,
    /// Available storyboards for the video
    pub storyboards: Vec<crate::types::VideoStoryboard>,
    /// Cover of the video; may be null if none
    pub cover: Option<crate::types::Photo>,
    /// Timestamp from which the video playing must start, in seconds
    pub start_timestamp: i32,
}

/// A message with a poll
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePoll {
    /// Information about the poll
    pub poll: crate::types::Poll,
    /// Description of the poll
    pub description: crate::types::FormattedText,
    /// Media attached to the poll; may be null if none. If present, currently, can be only of the types pollMediaAnimation, pollMediaAudio, pollMediaDocument, pollMediaLocation, pollMediaPhoto, pollMediaVenue, or pollMediaVideo
    pub media: Option<crate::enums::PollMedia>,
    /// True, if an option can be added to the poll using addPollOption
    pub can_add_option: bool,
}

/// A message with information about an added poll option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePollOptionAdded {
    /// Identifier of the message with the poll; can be an identifier of a deleted message or 0
    pub poll_message_id: i64,
    /// Identifier of the added option in the poll
    pub option_id: String,
    /// Text of the option; 1-100 characters; may contain only custom emoji entities
    pub text: crate::types::FormattedText,
}

/// A message with information about a deleted poll option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePollOptionDeleted {
    /// Identifier of the message with the poll; can be an identifier of a deleted message or 0
    pub poll_message_id: i64,
    /// Identifier of the deleted option in the poll
    pub option_id: String,
    /// Text of the option; 1-100 characters; may contain only custom emoji entities
    pub text: crate::types::FormattedText,
}

/// An animation
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaAnimation {
    /// The animation to be sent
    pub animation: crate::types::InputAnimation,
}

/// An audio
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaAudio {
    /// The audio to be sent
    pub audio: crate::types::InputAudio,
}

/// A document (general file)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaDocument {
    /// The document to be sent
    pub document: crate::types::InputDocument,
}

/// A link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaLink {
    /// URL of the link
    pub url: String,
}

/// A location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaLocation {
    /// Location to be sent
    pub location: crate::types::Location,
}

/// A photo
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaPhoto {
    /// Photo to be sent
    pub photo: crate::types::InputPhoto,
}

/// A venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaVenue {
    /// Venue to send
    pub venue: crate::types::Venue,
}

/// A video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaVideo {
    /// The video to be sent
    pub video: crate::types::InputVideo,
}

/// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessagePoll {
    /// Poll question; 1-255 characters (up to 300 characters for bots). Only custom emoji entities are allowed to be added and only by Premium users
    pub question: crate::types::FormattedText,
    /// List of poll answer options; 1-getOption("poll_answer_count_max") options
    pub options: Vec<crate::types::InputPollOption>,
    /// Poll description; pass null to use an empty description; 0-getOption("message_caption_length_max") characters
    pub description: Option<crate::types::FormattedText>,
    /// Media attached to the poll; pass null if none. Must be one of the following types: inputPollMediaAnimation, inputPollMediaAudio, inputPollMediaDocument, inputPollMediaLocation,
    /// inputPollMediaPhoto, inputPollMediaVenue, or inputPollMediaVideo without caption
    pub media: Option<crate::enums::InputPollMedia>,
    /// True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels
    pub is_anonymous: bool,
    /// True, if multiple answer options can be chosen simultaneously
    pub allows_multiple_answers: bool,
    /// True, if the poll can be answered multiple times
    pub allows_revoting: bool,
    /// True, if only the users that are members of the chat for more than a day will be able to vote; for channel chats only
    pub members_only: bool,
    /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be able to vote; for channel chats only. If empty, then all users can participate in the poll.
    /// There can be up to getOption("poll_country_count_max") chosen countries
    pub country_codes: Vec<String>,
    /// True, if poll options must be shown in a fixed random order
    pub shuffle_options: bool,
    /// True, if the poll results will appear only after the poll closes
    pub hide_results_until_closes: bool,
    /// Type of the poll
    #[serde(rename = "type")]
    pub r#type: crate::enums::InputPollType,
    /// Amount of time the poll will be active after creation, in seconds; 0-getOption("poll_open_period_max"); pass 0 if not specified
    pub open_period: i32,
    /// Point in time (Unix timestamp) when the poll will automatically be closed; must be 0-getOption("poll_open_period_max") seconds in the future; pass 0 if not specified
    pub close_date: i32,
    /// True, if the poll needs to be sent already closed; for bots only
    pub is_closed: bool,
}

/// Contains properties of a poll option and describes actions that can be done with the option right now
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollOptionProperties {
    /// True, if the option can be deleted using deletePollOption
    pub can_be_deleted: bool,
    /// True, if the poll option can be replied in the same chat and forum topic using inputMessageReplyToMessage
    pub can_be_replied: bool,
    /// True, if the poll option can be replied in another chat or forum topic using inputMessageReplyToExternalMessage
    pub can_be_replied_in_another_chat: bool,
    /// True, if a link can be generated for the poll option using getMessageLink
    pub can_get_link: bool,
}

/// Returns only poll messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterPoll {
}

/// Returns only messages with unread poll votes for the current user. When using this filter the results can't be additionally filtered by a query or by the sending user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterUnreadPollVote {
}

/// A poll in a message was stopped
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventPollStopped {
    /// The message with the poll
    pub message: crate::types::Message,
}

/// A message with a poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPoll {
    /// Poll question
    pub question: String,
    /// True, if the poll is regular and not in quiz mode
    pub is_regular: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// An option was added to a poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPollOptionAdded {
    /// Text of the option
    pub text: String,
}

/// A detailed statistics about poll votes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollVoteStatistics {
    /// A graph containing distribution of votes in the poll
    pub vote_graph: crate::enums::StatisticalGraph,
}

/// Unread votes were added or removed from a poll message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageContainsUnreadPollVotes {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// True, if the message is a poll message with unread votes
    pub contains_unread_poll_votes: bool,
    /// The new number of messages with unread poll votes in the chat
    pub unread_poll_vote_count: i32,
}

/// The chat unread_poll_vote_count has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatUnreadPollVoteCount {
    /// Chat identifier
    pub chat_id: i64,
    /// The number of messages with unread poll votes left in the chat
    pub unread_poll_vote_count: i32,
}

/// A poll was updated; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdatePoll {
    /// New data about the poll
    pub poll: crate::types::Poll,
}

/// A user changed the answer to a poll; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdatePollAnswer {
    /// Unique poll identifier
    #[serde_as(as = "DisplayFromStr")]
    pub poll_id: i64,
    /// Identifier of the message sender that changed the answer to the poll
    pub voter_id: crate::enums::MessageSender,
    /// Unique identifiers of answer options, that were chosen by the user
    pub option_ids: Vec<String>,
    /// 0-based identifiers of answer options, that were chosen by the user
    pub option_positions: Vec<i32>,
}

