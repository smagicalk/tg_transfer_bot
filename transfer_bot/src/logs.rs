//! 日志子系统初始化与格式化配置模块。
//!
//! - 非测试环境：同时输出到控制台（Stdout，彩色紧凑格式）与按大小滚动的本地日志文件（10MB 单文件，保留 10 代）；
//! - 测试环境：支持在多线程并行测试中安全重复初始化，并定向到测试标准输出缓冲区。

#[cfg(not(test))]
use tracing_appender::non_blocking;
#[cfg(not(test))]
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 异步文件日志追加器的 WorkerGuard 单例句柄。
///
/// 必须在进程生命周期内保持存活，否则后台写入线程被 Drop 会导致未刷盘的日志丢失。
#[cfg(not(test))]
static LOG_GUARD: once_cell::sync::OnceCell<tracing_appender::non_blocking::WorkerGuard> =
    once_cell::sync::OnceCell::new();

/// 初始化生产运行时环境下的全局 Tracing 日志跟踪器。
///
/// - 从环境变量 `RUST_LOG` 解析过滤等级；若未设置则默认采用 `info,sea_orm=warn,sqlx=warn,tokio=warn`，
///   避免数据库执行细节刷屏。
/// - 配置控制台层（带 ANSI 颜色、时间戳、线程号、行号）与滚动文件层（`tg_transfer.log`）。
#[cfg(not(test))]
pub fn init_tracing() {
    // 从环境变量读取日志过滤规则；没有则使用默认规则。
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // 默认只输出业务关键流程，避免 SeaORM SQL 与 TDLib 进度更新刷屏。
        // 需要排查细节时可通过 RUST_LOG 覆盖：
        // - RUST_LOG=transfer_bot=debug,info 查看消息过滤、按钮路由、发送回执
        // - RUST_LOG=transfer_bot=trace,info 查看 TDLib update 与文件进度
        tracing_subscriber::EnvFilter::new("info,sea_orm=warn,sqlx=warn,tokio=warn")
    });

    // 滚动日志文件：单文件 10MB，保留 10 个历史文件。
    let file_appender = file_rotate::FileRotate::new(
        "tg_transfer.log",
        file_rotate::suffix::AppendCount::new(10),
        file_rotate::ContentLimit::Bytes(10 * 1024 * 1024),
        file_rotate::compression::Compression::OnRotate(1),
        Some(
            std::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .to_owned(),
        ),
    );
    let (file_writer, guard) = non_blocking(file_appender);

    // 控制台日志层。
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(false)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_line_number(true)
        .with_file(false)
        .compact();

    // 文件日志层。
    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_line_number(true)
        .with_file(true)
        .compact();

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    // 保存 guard，避免异步日志在进程退出时丢失。
    LOG_GUARD.set(guard).ok();
}

/// 初始化单元与集成测试环境下的 Tracing 日志跟踪器。
///
/// 使用 `try_init()` 允许多个测试并发执行时安全重复调用，捕获输出至测试控制台。
#[cfg(test)]
pub fn init_tracing() {
    // 测试环境下允许重复初始化（避免并行测试 panic）。
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // 测试默认不打印 SQL debug；失败时再用 RUST_LOG=debug 打开细节。
        tracing_subscriber::EnvFilter::new("info,sea_orm=warn,sqlx=warn,tokio=warn")
    });

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_test_writer()
        .with_writer(std::io::stdout)
        .with_target(false)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_line_number(true)
        .with_file(false)
        .compact()
        .try_init();
}
