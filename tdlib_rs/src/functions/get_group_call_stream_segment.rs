use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a file with a segment of a video chat or live story in a modified OGG format for audio or MPEG-4 format for video
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `time_offset` - Point in time when the stream segment begins; Unix timestamp in milliseconds
/// * `scale` - Segment duration scale; 0-1. Segment's duration is 1000/(2**scale) milliseconds
/// * `channel_id` - Identifier of an audio/video channel to get as received from tgcalls
/// * `video_quality` - Video quality as received from tgcalls; pass null to get the worst available quality
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call_stream_segment(
    group_call_id: i32,
    time_offset: i64,
    scale: i32,
    channel_id: i32,
    video_quality: Option<crate::enums::GroupCallVideoQuality>,
    client_id: i32,
) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
    "@type": "getGroupCallStreamSegment",
    "group_call_id": group_call_id,
    "time_offset": time_offset,
    "scale": scale,
    "channel_id": channel_id,
    "video_quality": video_quality,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
