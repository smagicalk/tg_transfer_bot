#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An invoice from a link of the type internalLinkTypeInvoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceName {
    /// Name of the invoice
    pub name: String,
}
