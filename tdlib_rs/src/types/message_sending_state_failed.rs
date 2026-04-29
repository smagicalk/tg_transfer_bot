#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message failed to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendingStateFailed {
    /// The cause of the message sending failure
    pub error: crate::types::Error,
    /// True, if the message can be re-sent using resendMessages or readdQuickReplyShortcutMessages
    pub can_retry: bool,
    /// True, if the message can be re-sent only on behalf of a different sender
    pub need_another_sender: bool,
    /// True, if the message can be re-sent only if another quote is chosen in the message that is replied by the given message
    pub need_another_reply_quote: bool,
    /// True, if the message can be re-sent only if the message to be replied is removed. This will be done automatically by resendMessages
    pub need_drop_reply: bool,
    /// The number of Telegram Stars that must be paid to send the message; 0 if the current amount is correct
    pub required_paid_message_star_count: i64,
    /// Time left before the message can be re-sent, in seconds. No update is sent when this field changes
    pub retry_after: f64,
}
