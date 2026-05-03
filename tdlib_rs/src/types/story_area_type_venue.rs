#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An area pointing to a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeVenue {
    /// Information about the venue
    pub venue: crate::types::Venue,
}
