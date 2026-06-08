#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputInvoice {
    /// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
    #[serde(rename(serialize = "inputInvoiceMessage", deserialize = "inputInvoiceMessage"))]
    Message(crate::types::InputInvoiceMessage),
    /// An invoice from a link of the type internalLinkTypeInvoice
    #[serde(rename(serialize = "inputInvoiceName", deserialize = "inputInvoiceName"))]
    Name(crate::types::InputInvoiceName),
    /// An invoice for a payment toward Telegram; must not be used in the in-store apps
    #[serde(rename(
        serialize = "inputInvoiceTelegram",
        deserialize = "inputInvoiceTelegram"
    ))]
    Telegram(crate::types::InputInvoiceTelegram),
}
