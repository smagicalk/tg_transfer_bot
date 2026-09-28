//! 转存机器人可执行程序入口点。
//!
//! 负责构建多线程 Tokio 异步运行时，并设置足够大的工作线程调用栈大小，
//! 避免在深度嵌套的消息和 JSON 协议解析时发生线程栈溢出。

/// 应用程序主入口函数。
///
/// 初始化多线程 Tokio 运行时环境并启动主服务循环。
///
/// # 返回值
/// 成功退出返回 `Ok(())`，遇到未捕获的严重错误返回 `anyhow::Result`。
fn main() -> anyhow::Result<()> {
    // 构建 Tokio 多线程异步运行时
    tokio::runtime::Builder::new_multi_thread()
        // 启用所有驱动器（包括网络 IO 和定时器驱动）
        .enable_all()
        // 为工作线程设置 8MB 调用栈，防止复杂协议解析与深层调用栈溢出
        .thread_stack_size(transfer_bot::TOKIO_WORKER_STACK_SIZE)
        .build()?
        // 阻塞当前主线程并驱动根异步任务 transfer_bot::run()
        .block_on(transfer_bot::run())
}
