#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains an HTTP URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct HttpUrl {
    /// The URL
    pub url: String,
}
