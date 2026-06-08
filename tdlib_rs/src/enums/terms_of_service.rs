#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TermsOfService {
    /// Contains Telegram terms of service
    #[serde(rename(serialize = "termsOfService", deserialize = "termsOfService"))]
    TermsOfService(crate::types::TermsOfService),
}
