#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Options to be used when a message content is copied without reference to the original sender. Service messages, messages with messageInvoice, messagePaidMedia, messageGiveaway, or messageGiveawayWinners content can't be copied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCopyOptions {
    /// True, if content of the message needs to be copied without reference to the original sender. Always true if the message is forwarded to a secret chat or is local.
    /// Use messageProperties.can_be_copied and messageProperties.can_be_copied_to_secret_chat to check whether the message is suitable
    pub send_copy: bool,
    /// True, if media caption of the message copy needs to be replaced. Ignored if send_copy is false
    pub replace_caption: bool,
    /// New message caption; pass null to copy message without caption. Ignored if replace_caption is false
    pub new_caption: Option<crate::types::FormattedText>,
    /// True, if new caption must be shown above the media; otherwise, new caption must be shown below the media; not supported in secret chats. Ignored if replace_caption is false
    pub new_show_caption_above_media: bool,
}
