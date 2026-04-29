#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ValidatedOrderInfo {
    /// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
    #[serde(rename(serialize = "validatedOrderInfo", deserialize = "validatedOrderInfo"))]
    ValidatedOrderInfo(crate::types::ValidatedOrderInfo),
}
