// TDLib 文件识别与下载准备逻辑。
// 该模块负责把消息映射到稳定 file_key，并确保媒体文件已经落地到本地。

use super::types::{DownloadSeed, PreparedCacheMeta};
use crate::tgbot::TdError;

/// 从消息内容提取稳定 file_key（优先 remote.unique_id）。
pub(in crate::tgbot::transfer) fn extract_file_key(
    message: &tdlib_rs::types::Message,
) -> Option<String> {
    match &message.content {
        tdlib_rs::enums::MessageContent::MessageVideo(video) => {
            file_key_from_file(&video.video.video)
        }
        tdlib_rs::enums::MessageContent::MessageAudio(audio) => {
            file_key_from_file(&audio.audio.audio)
        }
        tdlib_rs::enums::MessageContent::MessageVoiceNote(voice) => {
            file_key_from_file(&voice.voice_note.voice)
        }
        tdlib_rs::enums::MessageContent::MessageDocument(document) => {
            file_key_from_file(&document.document.document)
        }
        tdlib_rs::enums::MessageContent::MessagePhoto(photo) => {
            let best = photo
                .photo
                .sizes
                .iter()
                .max_by_key(|s| (s.width as i64) * (s.height as i64));
            best.and_then(|s| file_key_from_file(&s.photo))
        }
        _ => None,
    }
}

/// 从消息内容提取下载种子。
/// 这里不要求文件已下载完成，只需要拿到 TDLib file_id 和预估大小。
pub(in crate::tgbot::transfer) fn extract_download_seed(
    message: &tdlib_rs::types::Message,
) -> Option<DownloadSeed> {
    let file = primary_file_from_message(message)?;
    let file_key = file_key_from_file(&file)?;
    let size_bytes = if file.size > 0 {
        Some(file.size)
    } else if file.expected_size > 0 {
        Some(file.expected_size)
    } else {
        None
    };

    Some(DownloadSeed {
        file_key,
        td_file_id: file.id,
        size_bytes,
    })
}

/// 仅确保媒体文件已落地到本地（用于 single-flight 下载去重）。
/// 非媒体消息直接返回 Ok，不参与下载协同。
pub(in crate::tgbot::transfer) async fn ensure_media_downloaded(
    message: &tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<()> {
    let file_id = match &message.content {
        tdlib_rs::enums::MessageContent::MessagePhoto(photo) => photo
            .photo
            .sizes
            .iter()
            .max_by_key(|s| (s.width as i64) * (s.height as i64))
            .map(|s| s.photo.id),
        tdlib_rs::enums::MessageContent::MessageVideo(video) => Some(video.video.video.id),
        tdlib_rs::enums::MessageContent::MessageDocument(document) => {
            Some(document.document.document.id)
        }
        tdlib_rs::enums::MessageContent::MessageAudio(audio) => Some(audio.audio.audio.id),
        tdlib_rs::enums::MessageContent::MessageVoiceNote(voice) => Some(voice.voice_note.voice.id),
        _ => None,
    };

    let Some(file_id) = file_id else {
        return Ok(());
    };

    let _ = ensure_local_file(file_id, client_id).await?;
    Ok(())
}

/// 准备媒体文件并返回：
/// 1. 可用于上传的 InputFile::Local
/// 2. 可回填到 file_cache 的元信息
pub(super) async fn prepare_media_file(
    original_file: &tdlib_rs::types::File,
    client_id: i32,
) -> anyhow::Result<(PreparedCacheMeta, tdlib_rs::enums::InputFile)> {
    let refreshed = ensure_local_file(original_file.id, client_id).await?;
    let file_key = file_key_from_file(&refreshed)
        .ok_or_else(|| anyhow::anyhow!("file missing remote unique id / remote id"))?;

    if refreshed.local.path.is_empty() {
        anyhow::bail!("downloaded file has empty local path");
    }

    let size = if refreshed.size > 0 {
        Some(refreshed.size)
    } else if refreshed.expected_size > 0 {
        Some(refreshed.expected_size)
    } else {
        None
    };

    let local_input = tdlib_rs::enums::InputFile::Local(tdlib_rs::types::InputFileLocal {
        path: refreshed.local.path.clone(),
    });

    Ok((
        PreparedCacheMeta {
            file_key,
            td_file_id: refreshed.id,
            local_path: refreshed.local.path,
            size_bytes: size,
        },
        local_input,
    ))
}

/// 提取消息主媒体文件。
/// 说明：
/// - 照片取最大尺寸图；
/// - 其他媒体直接取自身主文件。
fn primary_file_from_message(message: &tdlib_rs::types::Message) -> Option<tdlib_rs::types::File> {
    match &message.content {
        tdlib_rs::enums::MessageContent::MessagePhoto(photo) => photo
            .photo
            .sizes
            .iter()
            .max_by_key(|s| (s.width as i64) * (s.height as i64))
            .map(|s| s.photo.clone()),
        tdlib_rs::enums::MessageContent::MessageVideo(video) => Some(video.video.video.clone()),
        tdlib_rs::enums::MessageContent::MessageDocument(document) => {
            Some(document.document.document.clone())
        }
        tdlib_rs::enums::MessageContent::MessageAudio(audio) => Some(audio.audio.audio.clone()),
        tdlib_rs::enums::MessageContent::MessageVoiceNote(voice) => {
            Some(voice.voice_note.voice.clone())
        }
        _ => None,
    }
}

/// 确保文件已下载到本地路径：
/// - 若本地已存在直接返回
/// - 否则执行同步下载并刷新文件状态
async fn ensure_local_file(file_id: i32, client_id: i32) -> anyhow::Result<tdlib_rs::types::File> {
    let mut current = get_file_by_id(file_id, client_id).await?;
    if current.local.is_downloading_completed && !current.local.path.is_empty() {
        return Ok(current);
    }

    let downloaded = tdlib_rs::functions::download_file(current.id, 32, 0, 0, true, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::File::File(file_after_download) = downloaded;
    current = file_after_download;

    if current.local.is_downloading_completed && !current.local.path.is_empty() {
        return Ok(current);
    }

    // 某些情况下 download_file 返回后 local 信息仍未刷新，这里再拉一次 get_file。
    get_file_by_id(file_id, client_id).await
}

/// 封装 get_file 调用并统一错误转换。
async fn get_file_by_id(file_id: i32, client_id: i32) -> anyhow::Result<tdlib_rs::types::File> {
    let current_file = tdlib_rs::functions::get_file(file_id, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::File::File(file) = current_file;
    Ok(file)
}

/// 提取文件 key：优先 remote.unique_id，退化到 remote.id。
fn file_key_from_file(file: &tdlib_rs::types::File) -> Option<String> {
    if !file.remote.unique_id.is_empty() {
        return Some(file.remote.unique_id.clone());
    }
    if !file.remote.id.is_empty() {
        return Some(file.remote.id.clone());
    }
    None
}
