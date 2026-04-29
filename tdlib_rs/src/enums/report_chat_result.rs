#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportChatResult {
    /// The chat was reported successfully
    #[serde(rename(serialize = "reportChatResultOk", deserialize = "reportChatResultOk"))]
    Ok,
    /// The user must choose an option to report the chat and repeat request with the chosen option
    #[serde(rename(serialize = "reportChatResultOptionRequired", deserialize = "reportChatResultOptionRequired"))]
    OptionRequired(crate::types::ReportChatResultOptionRequired),
    /// The user must add additional text details to the report
    #[serde(rename(serialize = "reportChatResultTextRequired", deserialize = "reportChatResultTextRequired"))]
    TextRequired(crate::types::ReportChatResultTextRequired),
    /// The user must choose messages to report and repeat the reportChat request with the chosen messages
    #[serde(rename(serialize = "reportChatResultMessagesRequired", deserialize = "reportChatResultMessagesRequired"))]
    MessagesRequired,
}
