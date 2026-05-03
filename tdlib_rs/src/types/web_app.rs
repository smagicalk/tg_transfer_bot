#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WebApp {
    /// Web App short name
    pub short_name: String,
    /// Web App title
    pub title: String,
    /// Web App description
    pub description: String,
    /// Web App photo
    pub photo: crate::types::Photo,
    /// Web App animation; may be null
    pub animation: Option<crate::types::Animation>,
}
