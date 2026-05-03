use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Translates a text to the given language. If the current user is a Telegram Premium user, then text formatting is preserved
/// # Arguments
/// * `text` - Text to translate
/// * `to_language_code` - Language code of the language to which the message is translated. Must be one of
/// "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "zh-CN", "zh", "zh-Hans", "zh-TW", "zh-Hant", "co", "hr", "cs", "da", "nl", "en", "eo", "et",
/// "fi", "fr", "fy", "gl", "ka", "de", "el", "gu", "ht", "ha", "haw", "he", "iw", "hi", "hmn", "hu", "is", "ig", "id", "in", "ga", "it", "ja", "jv", "kn", "kk", "km", "rw", "ko",
/// "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "ny", "or", "ps", "fa", "pl", "pt", "pa", "ro", "ru", "sm", "gd", "sr",
/// "st", "sn", "sd", "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg", "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi", "cy", "xh", "yi", "ji", "yo", "zu"
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn translate_text(
    text: crate::types::FormattedText,
    to_language_code: String,
    client_id: i32,
) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
    "@type": "translateText",
    "text": text,
    "to_language_code": to_language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
