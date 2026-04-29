#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a bot, which can be added to attachment or side menu
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttachmentMenuBot {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// True, if the bot supports opening from attachment menu in the chat with the bot
    pub supports_self_chat: bool,
    /// True, if the bot supports opening from attachment menu in private chats with ordinary users
    pub supports_user_chats: bool,
    /// True, if the bot supports opening from attachment menu in private chats with other bots
    pub supports_bot_chats: bool,
    /// True, if the bot supports opening from attachment menu in basic group and supergroup chats
    pub supports_group_chats: bool,
    /// True, if the bot supports opening from attachment menu in channel chats
    pub supports_channel_chats: bool,
    /// True, if the user must be asked for the permission to send messages to the bot
    pub request_write_access: bool,
    /// True, if the bot was explicitly added by the user. If the bot isn't added, then on the first bot launch toggleBotIsAddedToAttachmentMenu must be called and the bot must be added or removed
    pub is_added: bool,
    /// True, if the bot must be shown in the attachment menu
    pub show_in_attachment_menu: bool,
    /// True, if the bot must be shown in the side menu
    pub show_in_side_menu: bool,
    /// True, if a disclaimer, why the bot is shown in the side menu, is needed
    pub show_disclaimer_in_side_menu: bool,
    /// Name for the bot in attachment menu
    pub name: String,
    /// Color to highlight selected name of the bot if appropriate; may be null
    pub name_color: Option<crate::types::AttachmentMenuBotColor>,
    /// Default icon for the bot in SVG format; may be null
    pub default_icon: Option<crate::types::File>,
    /// Icon for the bot in SVG format for the official iOS app; may be null
    pub ios_static_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official iOS app; may be null
    pub ios_animated_icon: Option<crate::types::File>,
    /// Icon for the bot in PNG format for the official iOS app side menu; may be null
    pub ios_side_menu_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official Android app; may be null
    pub android_icon: Option<crate::types::File>,
    /// Icon for the bot in SVG format for the official Android app side menu; may be null
    pub android_side_menu_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official native macOS app; may be null
    pub macos_icon: Option<crate::types::File>,
    /// Icon for the bot in PNG format for the official macOS app side menu; may be null
    pub macos_side_menu_icon: Option<crate::types::File>,
    /// Color to highlight selected icon of the bot if appropriate; may be null
    pub icon_color: Option<crate::types::AttachmentMenuBotColor>,
    /// Default placeholder for opened Web Apps in SVG format; may be null
    pub web_app_placeholder: Option<crate::types::File>,
}
