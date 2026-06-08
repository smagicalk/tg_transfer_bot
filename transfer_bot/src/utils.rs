use std::future::Future;
use std::num::NonZeroU16;

// 通用工具函数模块。
// 当前仅提供异步重试工具，保留给后续 TDLib 临时错误、网络抖动等场景复用。

// 异步重试执行器：
// - `times` 使用 NonZeroU16，类型层面禁止传入 0 次导致没有错误可返回。
// - 第一次调用立即执行，失败后再等待 1 秒重试，避免成功路径无意义延迟。
// - 全部失败时直接返回最后一次错误，不使用 unwrap，方便后续安全复用。
#[allow(dead_code)]
async fn retry<T, E, F, Fut>(times: NonZeroU16, mut func: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut remaining = times.get();
    loop {
        match func().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                remaining -= 1;
                if remaining == 0 {
                    return Err(e);
                }

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::retry;
    use std::num::NonZeroU16;
    use std::sync::atomic::{AtomicU16, Ordering};

    #[tokio::test(start_paused = true)]
    async fn retry_returns_first_success() {
        // 成功后不能继续执行后续尝试，避免重复调用带副作用的 TDLib 操作。
        let attempts = AtomicU16::new(0);

        let result = retry(
            NonZeroU16::new(3).expect("retry times must be non-zero"),
            || {
                let current = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                async move {
                    if current == 2 {
                        Ok("done")
                    } else {
                        Err("temporary")
                    }
                }
            },
        )
        .await;

        assert_eq!(result, Ok("done"));
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn retry_returns_last_error_after_all_attempts_fail() {
        // 全部失败时返回最后一次错误，调用方可以看到最接近最终状态的错误原因。
        let attempts = AtomicU16::new(0);

        let result = retry(
            NonZeroU16::new(2).expect("retry times must be non-zero"),
            || {
                let current = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                async move { Err::<(), _>(current) }
            },
        )
        .await;

        assert_eq!(result, Err(2));
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }
}
