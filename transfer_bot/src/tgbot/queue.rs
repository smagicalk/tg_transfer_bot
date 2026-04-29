// 下载协调模块入口。
// `singleflight` 负责同一文件只下载一次，`progress` 负责当前进程内的实时进度快照。

mod progress;
mod singleflight;

pub use progress::{get_download_progress, update_download_progress};
pub use singleflight::run_singleflight;
