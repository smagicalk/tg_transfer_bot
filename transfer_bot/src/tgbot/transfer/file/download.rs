// TDLib 文件识别与下载准备逻辑。
// 该模块负责把消息映射到稳定 file_key，并确保媒体文件已经落地到本地。

use super::types::{DownloadSeed, PreparedCacheMeta};
use crate::tgbot::TdError;
use crate::tgbot::transfer::store;

const DOWNLOAD_CONTROL_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

/// 从消息内容提取跨任务稳定的唯一文件键 `file_key`（优先取 `remote.unique_id`）。
///
/// 照片消息会自动选择分辨率最高的一档尺寸来提取对应文件的键。
///
/// # 参数
/// - `message`: 消息引用。
///
/// # 返回值
/// - `Some(String)`: 提取到的稳定唯一文件标识。
/// - `None`: 纯文本或不包含媒体文件的消息。
pub(in crate::tgbot::transfer) fn extract_file_key(
    message: &tdlib_rs::types::Message,
) -> Option<String> {
    match &message.content {
        tdlib_rs::enums::MessageContent::MessageAnimation(animation) => {
            file_key_from_file(&animation.animation.animation)
        }
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

/// 判断消息是否能被当前转存流程处理。
///
/// 自动转存收到非文本消息时会先用这个函数过滤，避免把贴纸、投票等暂不支持类型入库。
///
/// # 参数
/// - `message`: 待检查的 TDLib Message 引用。
///
/// # 返回值
/// - `true`: 支持转存（动画、音频、文档、照片、纯文本、视频、语音）。
/// - `false`: 暂不支持类型（贴纸、位置、联系人、投票等）。
pub(in crate::tgbot::transfer) fn is_transferable_message(
    message: &tdlib_rs::types::Message,
) -> bool {
    matches!(
        message.content,
        tdlib_rs::enums::MessageContent::MessageAnimation(_)
            | tdlib_rs::enums::MessageContent::MessageAudio(_)
            | tdlib_rs::enums::MessageContent::MessageDocument(_)
            | tdlib_rs::enums::MessageContent::MessagePhoto(_)
            | tdlib_rs::enums::MessageContent::MessageText(_)
            | tdlib_rs::enums::MessageContent::MessageVideo(_)
            | tdlib_rs::enums::MessageContent::MessageVoiceNote(_)
    )
}

/// 从消息内容中提取下载种子信息。
///
/// 这里不要求文件已下载完成，只需要拿到 TDLib file_id 和预估大小。
/// 用于任务入库阶段即可在 `file_cache` 表中创建初始索引。
///
/// # 参数
/// - `message`: 消息引用。
///
/// # 返回值
/// - `Some(DownloadSeed)`: 提取出的下载种子结构体。
/// - `None`: 无媒体内容时返回 None。
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

/// 仅确保媒体文件已落地到本地（用于 single-flight 下载去重与并发控制）。
///
/// 非媒体消息直接返回 `Ok(())`，不参与下载协同。
///
/// # 参数
/// - `message`: 待下载的消息对象。
/// - `client_id`: 执行下载的 TDLib 客户端 ID。
/// - `job_id`: 关联的任务 ID（在下载期间监听任务暂停或取消信号）。
pub(in crate::tgbot::transfer) async fn ensure_media_downloaded(
    message: &tdlib_rs::types::Message,
    client_id: i32,
    job_id: i64,
) -> anyhow::Result<()> {
    let file_id = match &message.content {
        tdlib_rs::enums::MessageContent::MessageAnimation(animation) => {
            Some(animation.animation.animation.id)
        }
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

    let _ = ensure_local_file_with_control(file_id, client_id, Some(job_id)).await?;
    Ok(())
}

/// 准备媒体文件并返回：
/// 1. 可回填到 `file_cache` 的完整元信息 `PreparedCacheMeta`；
/// 2. 可用于上传的本地输入文件枚举 `InputFile::Local`。
///
/// 若 `cached_meta` 已存在且本地物理文件依然存在，则直接命中复用，跳过 TDLib 下载调用。
///
/// # 参数
/// - `original_file`: 原始 TDLib File 结构体。
/// - `client_id`: TDLib 客户端 ID。
/// - `cached_meta`: 可选的已有缓存元数据。
pub(super) async fn prepare_media_file(
    original_file: &tdlib_rs::types::File,
    client_id: i32,
    cached_meta: Option<&PreparedCacheMeta>,
) -> anyhow::Result<(PreparedCacheMeta, tdlib_rs::enums::InputFile)> {
    if let Some(cached) = cached_meta
        && file_key_from_file(original_file).as_deref() == Some(cached.file_key.as_str())
        && std::path::Path::new(&cached.local_path).is_file()
    {
        tracing::debug!(file_key = %cached.file_key, "reusing ready local file cache");
        return Ok((
            cached.clone(),
            tdlib_rs::enums::InputFile::Local(Box::new(tdlib_rs::types::InputFileLocal {
                path: cached.local_path.clone(),
            })),
        ));
    }

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

    let local_input =
        tdlib_rs::enums::InputFile::Local(Box::new(tdlib_rs::types::InputFileLocal {
            path: refreshed.local.path.clone(),
        }));

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

/// 提取消息中的主要媒体文件结构体。
///
/// - 照片取最大分辨率的那一张图；
/// - 其他媒体直接取自身主体 `File` 对象。
fn primary_file_from_message(message: &tdlib_rs::types::Message) -> Option<tdlib_rs::types::File> {
    match &message.content {
        tdlib_rs::enums::MessageContent::MessageAnimation(animation) => {
            Some(animation.animation.animation.clone())
        }
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

/// 确保文件已下载到本地路径（无任务控制监听）：
/// - 若本地已存在直接返回；
/// - 否则执行同步下载并刷新文件状态。
async fn ensure_local_file(file_id: i32, client_id: i32) -> anyhow::Result<tdlib_rs::types::File> {
    ensure_local_file_with_control(file_id, client_id, None).await
}

/// 确保文件已下载到本地路径，并支持在下载过程中协同监听任务暂停/取消控制信号。
///
/// 循环中使用 `tokio::select!` 每 250ms 轮询一次数据库中任务的状态；
/// 若任务被暂停或取消，立即调用 TDLib 的 `cancel_download_file` 释放带宽与连接并中断下载。
///
/// # 参数
/// - `file_id`: TDLib 文件对象 ID。
/// - `client_id`: TDLib 客户端 ID。
/// - `job_id`: 可选关联任务 ID。
async fn ensure_local_file_with_control(
    file_id: i32,
    client_id: i32,
    job_id: Option<i64>,
) -> anyhow::Result<tdlib_rs::types::File> {
    let mut current = get_file_by_id(file_id, client_id).await?;
    if current.local.is_downloading_completed && !current.local.path.is_empty() {
        tracing::debug!(file_id, "tdlib file already available locally");
        return Ok(current);
    }

    tracing::debug!(
        file_id,
        size = current.size,
        expected_size = current.expected_size,
        "tdlib file download started"
    );
    let download = tdlib_rs::functions::download_file(current.id, 32, 0, 0, true, client_id);
    tokio::pin!(download);
    let downloaded = loop {
        if let Some(job_id) = job_id {
            tokio::select! {
                result = &mut download => break result.map_err(|e| anyhow::Error::new(TdError(e)))?,
                _ = tokio::time::sleep(DOWNLOAD_CONTROL_POLL_INTERVAL) => {
                    let Some(status) = store::get_job_status(job_id).await? else {
                        anyhow::bail!("job not found while downloading: {job_id}");
                    };
                    if matches!(
                        status.as_str(),
                        store::JOB_STATUS_PAUSED
                            | store::JOB_STATUS_CANCELLING
                            | store::JOB_STATUS_CANCEL_FINALIZING
                            | store::JOB_STATUS_CANCELLED
                    ) {
                        if let Err(err) = tdlib_rs::functions::cancel_download_file(
                            current.id,
                            false,
                            client_id,
                        )
                        .await
                        {
                            tracing::debug!(
                                job_id,
                                file_id = current.id,
                                error_code = err.code,
                                error_message = %err.message,
                                "tdlib cancelDownloadFile returned error"
                            );
                        }
                        anyhow::bail!("transfer job control requested during download: {status}");
                    }
                }
            }
        } else {
            break download.await.map_err(|e| anyhow::Error::new(TdError(e)))?;
        }
    };
    let tdlib_rs::enums::File::File(file_after_download) = downloaded;
    current = *file_after_download;

    if current.local.is_downloading_completed && !current.local.path.is_empty() {
        tracing::debug!(
            file_id,
            downloaded_size = current.local.downloaded_size,
            "tdlib file download completed"
        );
        return Ok(current);
    }

    // 某些情况下 download_file 返回后 local 信息仍未刷新，这里再拉一次 get_file。
    get_file_by_id(file_id, client_id).await
}

/// 封装 `get_file` 调用并统一错误转换为 `anyhow::Error(TdError)`。
///
/// # 参数
/// - `file_id`: TDLib 文件对象 ID。
/// - `client_id`: 客户端 ID。
async fn get_file_by_id(file_id: i32, client_id: i32) -> anyhow::Result<tdlib_rs::types::File> {
    let current_file = tdlib_rs::functions::get_file(file_id, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::File::File(file) = current_file;
    Ok(*file)
}

/// 从 TDLib File 对象中提取唯一缓存键：优先使用 `remote.unique_id`，若为空则退化使用 `remote.id`。
///
/// # 参数
/// - `file`: TDLib File 引用。
///
/// # 返回值
/// - `Some(String)`: 提取出的稳定文件标识。
/// - `None`: 两者均为空。
fn file_key_from_file(file: &tdlib_rs::types::File) -> Option<String> {
    if !file.remote.unique_id.is_empty() {
        return Some(file.remote.unique_id.clone());
    }
    if !file.remote.id.is_empty() {
        return Some(file.remote.id.clone());
    }
    None
}
