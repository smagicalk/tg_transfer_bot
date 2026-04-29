#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes gifts available for resale
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftsForResale {
    /// Total number of gifts found
    pub total_count: i32,
    /// The gifts
    pub gifts: Vec<crate::types::GiftForResale>,
    /// Available models; for searchGiftsForResale requests without offset and attributes only
    pub models: Vec<crate::types::UpgradedGiftModelCount>,
    /// Available symbols; for searchGiftsForResale requests without offset and attributes only
    pub symbols: Vec<crate::types::UpgradedGiftSymbolCount>,
    /// Available backdrops; for searchGiftsForResale requests without offset and attributes only
    pub backdrops: Vec<crate::types::UpgradedGiftBackdropCount>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
