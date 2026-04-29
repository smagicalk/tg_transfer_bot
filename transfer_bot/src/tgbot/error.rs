// TDLib 错误适配层：
// 将 `tdlib_rs::types::Error` 适配为标准 Error，便于 anyhow 统一处理。
use std::fmt;

#[derive(Debug)]
pub struct TdError(pub tdlib_rs::types::Error);

impl fmt::Display for TdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code={}, message={}", self.0.code, self.0.message)
    }
}

impl std::error::Error for TdError {}
