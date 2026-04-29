#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramParameters {
    /// Describes parameters of an affiliate program
    #[serde(rename(serialize = "affiliateProgramParameters", deserialize = "affiliateProgramParameters"))]
    AffiliateProgramParameters(crate::types::AffiliateProgramParameters),
}
