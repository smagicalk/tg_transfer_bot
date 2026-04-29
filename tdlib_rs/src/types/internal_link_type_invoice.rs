#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeInvoice {
    /// Name of the invoice
    pub invoice_name: String,
}
