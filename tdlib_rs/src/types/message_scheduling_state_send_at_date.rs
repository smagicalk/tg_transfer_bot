#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message will be sent at the specified date
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSchedulingStateSendAtDate {
    /// Point in time (Unix timestamp) when the message will be sent. The date must be within 367 days in the future
    pub send_date: i32,
    /// Period after which the message will be sent again; in seconds; 0 if never; for Telegram Premium users only; may be non-zero only in sendMessage and forwardMessages with one message requests;
    /// must be one of 0, 86400, 7 * 86400, 14 * 86400, 30 * 86400, 91 * 86400, 182 * 86400, 365 * 86400, or additionally 60, or 300 in the Test DC
    pub repeat_period: i32,
}
