#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether a file can be downloaded and saved locally by Web App request
/// # Arguments
/// * `bot_user_id` - Identifier of the bot, providing the Web App
/// * `file_name` - Name of the file
/// * `url` - URL of the file
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_web_app_file_download(bot_user_id: i64, file_name: String, url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkWebAppFileDownload",
        "bot_user_id": bot_user_id,
        "file_name": file_name,
        "url": url,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
