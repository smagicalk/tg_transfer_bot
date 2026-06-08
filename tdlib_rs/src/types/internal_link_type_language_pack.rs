#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link.
/// If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeLanguagePack {
    /// Language pack identifier
    pub language_pack_id: String,
}
