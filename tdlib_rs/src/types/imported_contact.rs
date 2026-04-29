#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a contact to import
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ImportedContact {
    /// Phone number of the user
    pub phone_number: String,
    /// First name of the user; 1-64 characters
    pub first_name: String,
    /// Last name of the user; 0-64 characters
    pub last_name: String,
    /// Note to add about the user; 0-getOption("user_note_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed;
    /// pass null to keep the current user's note
    pub note: crate::types::FormattedText,
}
