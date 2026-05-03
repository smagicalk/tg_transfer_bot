#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user launched a Web App using getWebAppLinkUrl
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonLaunchedWebApp {
    /// Information about the Web App
    pub web_app: crate::types::WebApp,
}
