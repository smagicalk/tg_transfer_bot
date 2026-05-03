#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a link preview
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreview {
    /// Original URL of the link
    pub url: String,
    /// URL to display
    pub display_url: String,
    /// Short name of the site (e.g., Google Docs, App Store)
    pub site_name: String,
    /// Title of the content
    pub title: String,
    /// Description of the content
    pub description: crate::types::FormattedText,
    /// Author of the content
    pub author: String,
    /// Type of the link preview
    pub r#type: crate::enums::LinkPreviewType,
    /// True, if size of media in the preview can be changed
    pub has_large_media: bool,
    /// True, if large media preview must be shown; otherwise, the media preview must be shown small and only the first frame must be shown for videos
    pub show_large_media: bool,
    /// True, if media must be shown above link preview description; otherwise, the media must be shown below the description
    pub show_media_above_description: bool,
    /// True, if there is no need to show an ordinary open URL confirmation, when opening the URL from the preview, because the URL is shown in the message text in clear
    pub skip_confirmation: bool,
    /// True, if the link preview must be shown above message text; otherwise, the link preview must be shown below the message text
    pub show_above_text: bool,
    /// Version of instant view (currently, can be 1 or 2) for the web page; 0 if none
    pub instant_view_version: i32,
}
