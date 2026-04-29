// 文件准备与上传内容构建模块：
// - `download`：提取稳定 file_key、下载种子、确保媒体落地本地
// - `content`：构建 send_message / send_message_album 需要的 InputMessageContent
// - `types`：对 workflow 暴露的准备结果和元信息类型

mod content;
mod download;
mod types;

#[cfg(test)]
mod tests;

pub(super) use content::prepare_upload_content;
pub(super) use download::{ensure_media_downloaded, extract_download_seed, extract_file_key};
pub(super) use types::{DownloadSeed, PreparedCacheMeta, PreparedUpload, UploadKind};
