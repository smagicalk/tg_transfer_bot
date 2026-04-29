#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with an invoice from a bot. Use getInternalLink with internalLinkTypeBotStart to share the invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageInvoice {
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
    /// Currency for the product price
    pub currency: String,
    /// Product total price in the smallest units of the currency
    pub total_amount: i64,
    /// Unique invoice bot start_parameter to be passed to getInternalLink
    pub start_parameter: String,
    /// True, if the invoice is a test invoice
    pub is_test: bool,
    /// True, if the shipping address must be specified
    pub need_shipping_address: bool,
    /// The identifier of the message with the receipt, after the product has been purchased
    pub receipt_message_id: i64,
    /// Extended media attached to the invoice; may be null if none
    pub paid_media: Option<crate::enums::PaidMedia>,
    /// Extended media caption; may be null if none
    pub paid_media_caption: Option<crate::types::FormattedText>,
}
