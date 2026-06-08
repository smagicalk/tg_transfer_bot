#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Invoice {
    /// Product invoice
    #[serde(rename(serialize = "invoice", deserialize = "invoice"))]
    Invoice(crate::types::Invoice),
}
