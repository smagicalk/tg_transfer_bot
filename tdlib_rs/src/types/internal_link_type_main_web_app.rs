#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App.
/// If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu,
/// show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu,
/// then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot.
/// Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMainWebApp {
    /// Username of the bot
    pub bot_username: String,
    /// Start parameter to be passed to getMainWebApp
    pub start_parameter: String,
    /// The mode to be passed to getMainWebApp
    pub mode: crate::enums::WebAppOpenMode,
}
