#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to the My Profile application page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMyProfilePage {
    /// Section of the page; may be one of
    /// "", "posts", "posts/all-stories", "posts/add-album", "gifts", "archived-posts"
    pub section: String,
}
