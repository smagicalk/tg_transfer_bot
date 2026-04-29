#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Related articles
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockRelatedArticles {
    /// Block header
    pub header: crate::enums::RichText,
    /// List of related articles
    pub articles: Vec<crate::types::PageBlockRelatedArticle>,
}
