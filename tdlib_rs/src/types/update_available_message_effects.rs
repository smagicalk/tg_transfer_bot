#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of available message effects has changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAvailableMessageEffects {
    /// The new list of available message effects from emoji reactions
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub reaction_effect_ids: Vec<i64>,
    /// The new list of available message effects from Premium stickers
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub sticker_effect_ids: Vec<i64>,
}
