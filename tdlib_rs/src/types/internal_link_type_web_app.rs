#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message.
/// Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed.
/// If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog
/// and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
/// Then, call getWebAppLinkUrl and open the returned URL as a Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeWebApp {
    /// Username of the bot that owns the Web App
    pub bot_username: String,
    /// Short name of the Web App
    pub web_app_short_name: String,
    /// Start parameter to be passed to getWebAppLinkUrl
    pub start_parameter: String,
    /// The mode in which the Web App must be opened
    pub mode: crate::enums::WebAppOpenMode,
}
