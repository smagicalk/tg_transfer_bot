#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CreatedBasicGroupChat {
    /// Contains information about a newly created basic group chat
    #[serde(rename(serialize = "createdBasicGroupChat", deserialize = "createdBasicGroupChat"))]
    CreatedBasicGroupChat(crate::types::CreatedBasicGroupChat),
}
