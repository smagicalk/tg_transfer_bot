//!
//! TDLib `file` domain functions.
//!
//! Types, enums, and functions for local and remote files, photos, audio, videos, and documents.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about a file. This is an offline method
///
/// # Arguments
///
/// * `file_id` - Identifier of the file to get
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_file(file_id: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "getFile",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a file by its remote identifier. This is an offline method. Can be used to register a URL as a file for further uploading, or sending as a message. Even if the request succeeds, the file can be used only if it is still accessible to the user.
/// For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
///
/// # Arguments
///
/// * `remote_file_id` - Remote identifier of the file to get
/// * `file_type` - File type; pass null if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_remote_file(remote_file_id: String, file_type: Option<crate::enums::FileType>, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "getRemoteFile",
        "remote_file_id": remote_file_id,
        "file_type": file_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for outgoing messages with content of the type messageDocument in all chats except secret chats. Returns the results in reverse chronological order
///
/// # Arguments
///
/// * `query` - Query to search for in document file name and message caption
/// * `limit` - The maximum number of messages to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_outgoing_document_messages(query: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchOutgoingDocumentMessages",
        "query": query,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns advertisements to be shown while a video from a message is watched. Available only if messageProperties.can_get_video_advertisements
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::VideoMessageAdvertisements)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_video_message_advertisements(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::VideoMessageAdvertisements, crate::types::Error> {
    let request = json!({
        "@type": "getVideoMessageAdvertisements",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that the user viewed a video message advertisement
///
/// # Arguments
///
/// * `advertisement_unique_id` - Unique identifier of the advertisement
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn view_video_message_advertisement(advertisement_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewVideoMessageAdvertisement",
        "advertisement_unique_id": advertisement_unique_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the user clicked a video message advertisement
///
/// # Arguments
///
/// * `advertisement_unique_id` - Unique identifier of the advertisement
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn click_video_message_advertisement(advertisement_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickVideoMessageAdvertisement",
        "advertisement_unique_id": advertisement_unique_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports a video message advertisement to Telegram moderators
///
/// # Arguments
///
/// * `advertisement_unique_id` - Unique identifier of the advertisement
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReportSponsoredResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_video_message_advertisement(advertisement_unique_id: i64, option_id: String, client_id: i32) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
        "@type": "reportVideoMessageAdvertisement",
        "advertisement_unique_id": advertisement_unique_id,
        "option_id": option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. Can be called synchronously
///
/// # Arguments
///
/// * `file_name` - The name of the file or path to the file
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_file_mime_type(file_name: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getFileMimeType",
        "file_name": file_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. Can be called synchronously
///
/// # Arguments
///
/// * `mime_type` - The MIME type of the file
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_file_extension(mime_type: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getFileExtension",
        "mime_type": mime_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes potentially dangerous characters from the name of a file. Returns an empty string on failure. Can be called synchronously
///
/// # Arguments
///
/// * `file_name` - File name or path to the file
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clean_file_name(file_name: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "cleanFileName",
        "file_name": file_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that an audio was listened by the user
///
/// # Arguments
///
/// * `audio_file_id` - Identifier of the file with an audio
/// * `duration` - Duration of the listening to the audio, in seconds
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn listen_to_audio(audio_file_id: i32, duration: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "listenToAudio",
        "audio_file_id": audio_file_id,
        "duration": duration,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info member right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `photo` - New chat photo; pass null to delete the chat photo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_photo(chat_id: i64, photo: Option<crate::enums::InputChatPhoto>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPhoto",
        "chat_id": chat_id,
        "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes accent color and background custom emoji for profile of a supergroup or channel chat. Requires can_change_info administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `profile_accent_color_id` - Identifier of the accent color to use for profile; pass -1 if none. The chat must have at least profileAccentColor.min_supergroup_chat_boost_level for supergroups
/// or profileAccentColor.min_channel_chat_boost_level for channels boost level to pass the corresponding color
/// * `profile_background_custom_emoji_id` - Identifier of a custom emoji to be shown on the chat's profile photo background; 0 if none. Use chatBoostLevelFeatures.can_set_profile_background_custom_emoji to check whether a custom emoji can be set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_profile_accent_color(chat_id: i64, profile_accent_color_id: i32, profile_background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatProfileAccentColor",
        "chat_id": chat_id,
        "profile_accent_color_id": profile_accent_color_id,
        "profile_background_custom_emoji_id": profile_background_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
///
/// # Arguments
///
/// * `file_id` - Identifier of the file to download
/// * `priority` - Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first
/// * `offset` - The starting position from which the file needs to be downloaded
/// * `limit` - Number of bytes which need to be downloaded starting from the "offset" position before the download will automatically be canceled; use 0 to download without a limit
/// * `synchronous` - Pass true to return response only after the file download has succeeded, has failed, has been canceled, or a new downloadFile request with different offset/limit parameters was sent; pass false to return file state immediately, just after the download has been started
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn download_file(file_id: i32, priority: i32, offset: i64, limit: i64, synchronous: bool, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "downloadFile",
        "file_id": file_id,
        "priority": priority,
        "offset": offset,
        "limit": limit,
        "synchronous": synchronous,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns file downloaded prefix size from a given offset, in bytes
///
/// # Arguments
///
/// * `file_id` - Identifier of the file
/// * `offset` - Offset from which downloaded prefix size needs to be calculated
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FileDownloadedPrefixSize)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_file_downloaded_prefix_size(file_id: i32, offset: i64, client_id: i32) -> Result<crate::enums::FileDownloadedPrefixSize, crate::types::Error> {
    let request = json!({
        "@type": "getFileDownloadedPrefixSize",
        "file_id": file_id,
        "offset": offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Stops the downloading of a file. If a file has already been downloaded, does nothing
///
/// # Arguments
///
/// * `file_id` - Identifier of a file to stop downloading
/// * `only_if_pending` - Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn cancel_download_file(file_id: i32, only_if_pending: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "cancelDownloadFile",
        "file_id": file_id,
        "only_if_pending": only_if_pending,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns suggested name for saving a file in a given directory
///
/// # Arguments
///
/// * `file_id` - Identifier of the file
/// * `directory` - Directory in which the file is expected to be saved
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_suggested_file_name(file_id: i32, directory: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getSuggestedFileName",
        "file_id": file_id,
        "directory": directory,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Preliminarily uploads a file to the cloud before sending it in a message, which can be useful for uploading of being recorded voice and video notes.
/// In all other cases there is no need to preliminary upload a file. Updates updateFile will be used to notify about upload progress.
/// The upload will not be completed until the file is sent in a message
///
/// # Arguments
///
/// * `file` - File to upload
/// * `file_type` - File type; pass null if unknown
/// * `priority` - Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which preliminaryUploadFile was called will be uploaded first
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn preliminary_upload_file(file: crate::enums::InputFile, file_type: Option<crate::enums::FileType>, priority: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "preliminaryUploadFile",
        "file": file,
        "file_type": file_type,
        "priority": priority,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Stops the preliminary uploading of a file. Supported only for files uploaded by using preliminaryUploadFile
///
/// # Arguments
///
/// * `file_id` - Identifier of the file to stop uploading
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn cancel_preliminary_upload_file(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "cancelPreliminaryUploadFile",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Writes a part of a generated file. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
///
/// # Arguments
///
/// * `generation_id` - The identifier of the generation process
/// * `offset` - The offset from which to write the data to the file
/// * `data` - The data to write
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn write_generated_file_part(generation_id: i64, offset: i64, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "writeGeneratedFilePart",
        "generation_id": generation_id,
        "offset": offset,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib on a file generation progress
///
/// # Arguments
///
/// * `generation_id` - The identifier of the generation process
/// * `expected_size` - Expected size of the generated file, in bytes; 0 if unknown
/// * `local_prefix_size` - The number of bytes already generated
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_file_generation_progress(generation_id: i64, expected_size: i64, local_prefix_size: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setFileGenerationProgress",
        "generation_id": generation_id,
        "expected_size": expected_size,
        "local_prefix_size": local_prefix_size,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Finishes the file generation
///
/// # Arguments
///
/// * `generation_id` - The identifier of the generation process
/// * `error` - If passed, the file generation has failed and must be terminated; pass null if the file generation succeeded
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn finish_file_generation(generation_id: i64, error: Option<crate::types::Error>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "finishFileGeneration",
        "generation_id": generation_id,
        "error": error,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
///
/// # Arguments
///
/// * `file_id` - Identifier of the file. The file must be located in the TDLib file cache
/// * `offset` - The offset from which to read the file
/// * `count` - Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Data)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_file_part(file_id: i32, offset: i64, count: i64, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "readFilePart",
        "file_id": file_id,
        "offset": offset,
        "count": count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a file from the TDLib file cache
///
/// # Arguments
///
/// * `file_id` - Identifier of the file to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_file(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteFile",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a file from a message to the list of file downloads. Download progress and completion of the download will be notified through updateFile updates.
/// If message database is used, the list of file downloads is persistent across application restarts. The downloading is independent of download using downloadFile, i.e. it continues if downloadFile is canceled or is used to download a part of the file
///
/// # Arguments
///
/// * `file_id` - Identifier of the file to download
/// * `chat_id` - Chat identifier of the message with the file
/// * `message_id` - Message identifier
/// * `priority` - Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_file_to_downloads(file_id: i32, chat_id: i64, message_id: i64, priority: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "addFileToDownloads",
        "file_id": file_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "priority": priority,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a file from the file download list
///
/// # Arguments
///
/// * `file_id` - Identifier of the downloaded file
/// * `delete_from_cache` - Pass true to delete the file from the TDLib file cache
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_file_from_downloads(file_id: i32, delete_from_cache: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeFileFromDownloads",
        "file_id": file_id,
        "delete_from_cache": delete_from_cache,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all files from the file download list
///
/// # Arguments
///
/// * `only_active` - Pass true to remove only active downloads, including paused
/// * `only_completed` - Pass true to remove only completed downloads
/// * `delete_from_cache` - Pass true to delete the file from the TDLib file cache
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_all_files_from_downloads(only_active: bool, only_completed: bool, delete_from_cache: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeAllFilesFromDownloads",
        "only_active": only_active,
        "only_completed": only_completed,
        "delete_from_cache": delete_from_cache,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches for files in the file download list or recently downloaded files from the list
///
/// # Arguments
///
/// * `query` - Query to search for; may be empty to return all downloaded files
/// * `only_active` - Pass true to search only for active downloads, including paused
/// * `only_completed` - Pass true to search only for completed downloads
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of files to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundFileDownloads)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_file_downloads(query: String, only_active: bool, only_completed: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundFileDownloads, crate::types::Error> {
    let request = json!({
        "@type": "searchFileDownloads",
        "query": query,
        "only_active": only_active,
        "only_completed": only_completed,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a file with messages exported from another application
///
/// # Arguments
///
/// * `message_file_head` - Beginning of the message file; up to 100 first lines
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageFileType)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_file_type(message_file_head: String, client_id: i32) -> Result<crate::enums::MessageFileType, crate::types::Error> {
    let request = json!({
        "@type": "getMessageFileType",
        "message_file_head": message_file_head,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes a personal profile photo of a contact user
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `photo` - Profile photo to set; pass null to delete the photo; inputChatPhotoPrevious isn't supported in this function
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_user_personal_profile_photo(user_id: i64, photo: Option<crate::enums::InputChatPhoto>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserPersonalProfilePhoto",
        "user_id": user_id,
        "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Suggests a profile photo to another regular user with common messages and allowing non-paid messages
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `photo` - Profile photo to suggest; inputChatPhotoPrevious isn't supported in this function
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn suggest_user_profile_photo(user_id: i64, photo: crate::enums::InputChatPhoto, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "suggestUserProfilePhoto",
        "user_id": user_id,
        "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the profile photos of a user. Personal and public photo aren't returned
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `offset` - The number of photos to skip; must be non-negative
/// * `limit` - The maximum number of photos to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatPhotos)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_profile_photos(user_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::ChatPhotos, crate::types::Error> {
    let request = json!({
        "@type": "getUserProfilePhotos",
        "user_id": user_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of profile audio files of a user
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `offset` - The number of audio files to skip; must be non-negative
/// * `limit` - The maximum number of audio files to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Audios)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_profile_audios(user_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Audios, crate::types::Error> {
    let request = json!({
        "@type": "getUserProfileAudios",
        "user_id": user_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a file is in the profile audio files of the current user. Returns a 404 error if it isn't
///
/// # Arguments
///
/// * `file_id` - Identifier of the audio file to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn is_profile_audio(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "isProfileAudio",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds an audio file to the beginning of the profile audio files of the current user
///
/// # Arguments
///
/// * `audio` - The audio to add
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_profile_audio(audio: crate::types::InputAudio, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addProfileAudio",
        "audio": audio,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes position of an audio file in the profile audio files of the current user
///
/// # Arguments
///
/// * `file_id` - Identifier of the file from profile audio files, which position will be changed
/// * `after_file_id` - Identifier of the file from profile audio files after which the file will be positioned; pass 0 to move the file to the beginning of the list
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_profile_audio_position(file_id: i32, after_file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setProfileAudioPosition",
        "file_id": file_id,
        "after_file_id": after_file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes an audio file from the profile audio files of the current user
///
/// # Arguments
///
/// * `file_id` - Identifier of the audio file to be removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_profile_audio(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeProfileAudio",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns saved animations
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Animations)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_animations(client_id: i32) -> Result<crate::enums::Animations, crate::types::Error> {
    let request = json!({
        "@type": "getSavedAnimations",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first.
/// Only non-secret video animations with MIME type "video/mp4" can be added to the list
///
/// # Arguments
///
/// * `animation` - The animation file to be added. Only animations known to the server (i.e., successfully sent via a message) can be added to the list
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_saved_animation(animation: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addSavedAnimation",
        "animation": animation,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes an animation from the list of saved animations
///
/// # Arguments
///
/// * `animation` - Animation file to be removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_saved_animation(animation: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeSavedAnimation",
        "animation": animation,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes a profile photo for the current user
///
/// # Arguments
///
/// * `photo` - Profile photo to set
/// * `is_public` - Pass true to set the public photo, which will be visible even if the main photo is hidden by privacy settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_profile_photo(photo: crate::enums::InputChatPhoto, is_public: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setProfilePhoto",
        "photo": photo,
        "is_public": is_public,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a profile photo
///
/// # Arguments
///
/// * `profile_photo_id` - Identifier of the profile photo to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_profile_photo(profile_photo_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteProfilePhoto",
        "profile_photo_id": profile_photo_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes accent color and background custom emoji for profile of the current user; for Telegram Premium users only
///
/// # Arguments
///
/// * `profile_accent_color_id` - Identifier of the accent color to use for profile; pass -1 if none
/// * `profile_background_custom_emoji_id` - Identifier of a custom emoji to be shown on the user's profile photo background; 0 if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_profile_accent_color(profile_accent_color_id: i32, profile_background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setProfileAccentColor",
        "profile_accent_color_id": profile_accent_color_id,
        "profile_background_custom_emoji_id": profile_background_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the main profile tab of the current user
///
/// # Arguments
///
/// * `main_profile_tab` - The new value of the main profile tab
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_main_profile_tab(main_profile_tab: crate::enums::ProfileTab, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMainProfileTab",
        "main_profile_tab": main_profile_tab,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the main profile tab of the channel; requires can_change_info administrator right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the channel
/// * `main_profile_tab` - The new value of the main profile tab
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_main_profile_tab(supergroup_id: i64, main_profile_tab: crate::enums::ProfileTab, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupMainProfileTab",
        "supergroup_id": supergroup_id,
        "main_profile_tab": main_profile_tab,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns promotional animation for upgraded gifts
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Animation)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gifts_promotional_animation(client_id: i32) -> Result<crate::enums::Animation, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftsPromotionalAnimation",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Reports a chat photo to the Telegram moderators. A chat photo can be reported only if chat.can_be_reported
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `file_id` - Identifier of the photo to report. Only full photos from chatPhoto can be reported
/// * `reason` - The reason for reporting the chat photo
/// * `text` - Additional report details; 0-1024 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_chat_photo(chat_id: i64, file_id: i32, reason: crate::enums::ReportReason, text: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportChatPhoto",
        "chat_id": chat_id,
        "file_id": file_id,
        "reason": reason,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
///
/// # Arguments
///
/// * `location` - Location of the map center
/// * `zoom` - Map zoom level; 13-20
/// * `width` - Map width in pixels before applying scale; 16-1024
/// * `height` - Map height in pixels before applying scale; 16-1024
/// * `scale` - Map scale; 1-3
/// * `chat_id` - Identifier of a chat in which the thumbnail will be shown. Use 0 if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_map_thumbnail_file(location: crate::types::Location, zoom: i32, width: i32, height: i32, scale: i32, chat_id: i64, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "getMapThumbnailFile",
        "location": location,
        "zoom": zoom,
        "width": width,
        "height": height,
        "scale": scale,
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

