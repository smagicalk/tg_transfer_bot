#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MaskPoint {
    /// The mask is placed relatively to the forehead
    #[serde(rename(serialize = "maskPointForehead", deserialize = "maskPointForehead"))]
    Forehead,
    /// The mask is placed relatively to the eyes
    #[serde(rename(serialize = "maskPointEyes", deserialize = "maskPointEyes"))]
    Eyes,
    /// The mask is placed relatively to the mouth
    #[serde(rename(serialize = "maskPointMouth", deserialize = "maskPointMouth"))]
    Mouth,
    /// The mask is placed relatively to the chin
    #[serde(rename(serialize = "maskPointChin", deserialize = "maskPointChin"))]
    Chin,
}
