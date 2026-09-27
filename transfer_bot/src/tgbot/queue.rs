//! 下载排队与并发协调模块。
//!
//! # 核心职责
//! 1. **单飞防并发击穿（Singleflight）**：通过 `singleflight` 子模块确保同一远程文件在同一时刻仅有一个下载协程在运行，其他相同请求共享并等待其完成结果。
//! 2. **下载进度快照（Progress）**：通过 `progress` 子模块提供进程内毫秒级更新的文件下载进度快照查询，供前端命令（如 `/downloads`）及 UI 实时渲染。

mod progress;
mod singleflight;

pub use progress::get_download_progress;
pub use singleflight::run_singleflight;

