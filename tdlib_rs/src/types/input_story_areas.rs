#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of story areas to be added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreas {
    /// List of input story areas. Currently, a story can have
    /// up to 10 inputStoryAreaTypeLocation, inputStoryAreaTypeFoundVenue, and inputStoryAreaTypePreviousVenue areas,
    /// up to getOption("story_suggested_reaction_area_count_max") inputStoryAreaTypeSuggestedReaction areas,
    /// up to 1 inputStoryAreaTypeMessage area,
    /// up to getOption("story_link_area_count_max") inputStoryAreaTypeLink areas if the current user is a Telegram Premium user,
    /// up to 3 inputStoryAreaTypeWeather areas, and
    /// up to 1 inputStoryAreaTypeUpgradedGift area
    pub areas: Vec<crate::types::InputStoryArea>,
}
