#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A forum topic has been created
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// True, if the name of the topic wasn't added explicitly
    pub is_name_implicit: bool,
    /// Icon of the topic
    pub icon: crate::types::ForumTopicIcon,
}
