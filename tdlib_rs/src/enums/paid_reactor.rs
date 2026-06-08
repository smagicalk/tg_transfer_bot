#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidReactor {
    /// Contains information about a user who added paid reactions
    #[serde(rename(serialize = "paidReactor", deserialize = "paidReactor"))]
    PaidReactor(crate::types::PaidReactor),
}
