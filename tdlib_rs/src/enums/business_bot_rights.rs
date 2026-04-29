#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessBotRights {
    /// Describes rights of a business bot
    #[serde(rename(serialize = "businessBotRights", deserialize = "businessBotRights"))]
    BusinessBotRights(crate::types::BusinessBotRights),
}
