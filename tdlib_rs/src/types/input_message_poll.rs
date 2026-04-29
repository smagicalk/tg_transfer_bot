#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessagePoll {
    /// Poll question; 1-255 characters (up to 300 characters for bots). Only custom emoji entities are allowed to be added and only by Premium users
    pub question: crate::types::FormattedText,
    /// List of poll answer options, 2-getOption("poll_answer_count_max") strings 1-100 characters each. Only custom emoji entities are allowed to be added and only by Premium users
    pub options: Vec<crate::types::FormattedText>,
    /// True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels
    pub is_anonymous: bool,
    /// Type of the poll
    pub r#type: crate::enums::PollType,
}
