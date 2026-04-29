#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains the call identifier
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallId {
    /// Call identifier
    pub id: i32,
}
