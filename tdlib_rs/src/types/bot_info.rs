#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotInfo {
    /// The text that is shown on the bot's profile page and is sent together with the link when users share the bot
    pub short_description: String,
    /// The text shown in the chat with the bot if the chat is empty
    pub description: String,
    /// Photo shown in the chat with the bot if the chat is empty; may be null
    pub photo: Option<crate::types::Photo>,
    /// Animation shown in the chat with the bot if the chat is empty; may be null
    pub animation: Option<crate::types::Animation>,
    /// Information about a button to show instead of the bot commands menu button; may be null if ordinary bot commands menu must be shown
    pub menu_button: Option<crate::types::BotMenuButton>,
    /// List of the bot commands
    pub commands: Vec<crate::types::BotCommand>,
    /// The HTTP link to the privacy policy of the bot. If empty, then /privacy command must be used if supported by the bot. If the command isn't supported, then https:telegram.org/privacy-tpa must be opened
    pub privacy_policy_url: String,
    /// Default administrator rights for adding the bot to basic group and supergroup chats; may be null
    pub default_group_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Default administrator rights for adding the bot to channels; may be null
    pub default_channel_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Information about the affiliate program of the bot; may be null if none
    pub affiliate_program: Option<crate::types::AffiliateProgramInfo>,
    /// Default light background color for bot Web Apps; -1 if not specified
    pub web_app_background_light_color: i32,
    /// Default dark background color for bot Web Apps; -1 if not specified
    pub web_app_background_dark_color: i32,
    /// Default light header color for bot Web Apps; -1 if not specified
    pub web_app_header_light_color: i32,
    /// Default dark header color for bot Web Apps; -1 if not specified
    pub web_app_header_dark_color: i32,
    /// Parameters of the verification that can be provided by the bot; may be null if none or the current user isn't the owner of the bot
    pub verification_parameters: Option<crate::types::BotVerificationParameters>,
    /// True, if the bot's revenue statistics are available to the current user
    pub can_get_revenue_statistics: bool,
    /// True, if the bot can manage emoji status of the current user
    pub can_manage_emoji_status: bool,
    /// True, if the bot has media previews
    pub has_media_previews: bool,
    /// The internal link, which can be used to edit bot commands; may be null
    pub edit_commands_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit bot description; may be null
    pub edit_description_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit the photo or animation shown in the chat with the bot if the chat is empty; may be null
    pub edit_description_media_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit bot settings; may be null
    pub edit_settings_link: Option<crate::enums::InternalLinkType>,
}
