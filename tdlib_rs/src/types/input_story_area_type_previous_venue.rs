#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area pointing to a venue already added to the story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypePreviousVenue {
    /// Provider of the venue
    pub venue_provider: String,
    /// Identifier of the venue in the provider database
    pub venue_id: String,
}
