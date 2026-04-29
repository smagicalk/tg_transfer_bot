#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with an invoice; can be used only by bots
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageInvoice {
    /// Invoice
    pub invoice: crate::types::Invoice,
    /// Product title; 1-32 characters
    pub title: String,
    /// Product description; 0-255 characters
    pub description: String,
    /// Product photo URL; optional
    pub photo_url: String,
    /// Product photo size
    pub photo_size: i32,
    /// Product photo width
    pub photo_width: i32,
    /// Product photo height
    pub photo_height: i32,
    /// The invoice payload
    pub payload: String,
    /// Payment provider token; may be empty for payments in Telegram Stars
    pub provider_token: String,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider
    pub provider_data: String,
    /// Unique invoice bot deep link parameter for the generation of this invoice. If empty, it would be possible to pay directly from forwards of the invoice message
    pub start_parameter: String,
    /// The content of paid media attached to the invoice; pass null if none
    pub paid_media: Option<crate::types::InputPaidMedia>,
    /// Paid media caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub paid_media_caption: Option<crate::types::FormattedText>,
}
