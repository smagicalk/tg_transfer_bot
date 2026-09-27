//! 命令行参数解析与配置加解密模块。
//!
//! 该模块使用 `clap` 库解析转存机器人的启动参数，支持：
//! 1. 通过 `-c / --config` 指定 JSON 配置文件路径；
//! 2. 可选子命令 `encrypt`（或别名 `enc`）对配置文件进行高强度对称加密导出；
//! 3. 可选子命令 `decrypt`（或别名 `dec`）在内存中解密配置文件并启动。

use clap::{Parser, Subcommand};
use std::process::exit;

/// 转存机器人命令行参数结构体。
///
/// 封装运行机器人所需的全部命令行参数以及可执行的子命令。
#[derive(Parser, Debug)]
#[command(name = "cli", version, about = "转存机器人命令行入口")]
pub(crate) struct TransferBotCli {
    /// 配置文件路径（必填项）。
    ///
    /// 可以是明文 JSON 文件路径（如 `config.json`），
    /// 也可以是密文文件路径（与 `decrypt` 子命令配合使用）。
    #[arg(
        short = 'c',
        long = "config",
        required = true,
        help = "配置文件路径（必填）"
    )]
    pub config: String,

    /// 可选运行模式或子命令。
    ///
    /// 当为空或为 `Mode::None` 时按常规模式启动服务；
    /// 当为 `Mode::Encrypt` 或 `Mode::Decrypt` 时执行对应的密码学编解码操作。
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

/// 命令行执行模式枚举。
///
/// 定义程序是正常启动还是执行配置文件的加密/解密运维操作。
#[derive(Subcommand, Debug)]
pub enum Mode {
    /// 默认模式：不加密也不解密，直接读取明文配置文件并启动机器人。
    #[command(about = "默认模式（不加密/不解密）", hide = true)]
    None,

    /// 加密配置文件模式。
    ///
    /// 读取指定的配置文件明文，使用传入的密码加密并保存为 `<config>.enc` 文件，然后退出程序。
    #[command(about = "加密并指定密码", visible_alias = "enc")]
    Encrypt {
        /// 用于加密配置文件的密码字符串
        password: String,
    },

    /// 解密配置文件模式。
    ///
    /// 读取指定的密文文件，使用密码在内存中解密出明文字符串，供机器人后续载入。
    #[command(about = "解密并指定密码", visible_alias = "dec")]
    Decrypt {
        /// 用于解密配置文件的密码字符串
        password: String,
    },
}

impl TransferBotCli {
    /// 异步读取文件并返回文本内容。
    ///
    /// # 参数
    /// - `path`: 要读取的文件绝对或相对路径
    ///
    /// # 返回值
    /// - `Ok(String)`: 读取到的文件全部内容字符串
    /// - `Err(anyhow::Error)`: 文件不存在、指定路径是目录或发生底层 IO 错误
    async fn read_file(path: &std::path::PathBuf) -> anyhow::Result<String> {
        // 校验文件是否存在
        if !path.exists() {
            anyhow::bail!("{path:?} 文件不存在")
        }
        // 校验目标是否为文件而非目录
        if path.is_dir() {
            anyhow::bail!("{path:?} 是目录，不是文件")
        }
        // 异步读取文件完整文本
        Ok(tokio::fs::read_to_string(path).await?)
    }

    /// 统一获取配置文件的明文内容。
    ///
    /// 根据当前命令行的模式设置执行不同逻辑：
    /// - `None` 或 `Mode::None`: 直接以明文方式从磁盘读取配置；
    /// - `Mode::Encrypt`: 将配置文件加密后生成 `.enc` 文件并立即退出进程（`exit(0)`）；
    /// - `Mode::Decrypt`: 使用密码在内存中解密密文文件并返回明文字符串，不向磁盘写出未加密文件。
    ///
    /// # 返回值
    /// 返回待解析为 `BotConfig` 的 JSON 格式明文字符串。
    pub async fn get_config(&self) -> anyhow::Result<String> {
        let path = std::path::PathBuf::from(&self.config);
        match &self.mode {
            // 未指定子命令，按明文文件读取
            None => Self::read_file(&path).await,
            Some(mode) => match mode {
                // 显式指定 None 模式，按明文文件读取
                Mode::None => Self::read_file(&path).await,
                // 加密模式：生成密文文件后正常终止进程
                Mode::Encrypt { password } => {
                    let mut save = self.config.clone();
                    save.push_str(".enc");
                    crate::crypto::encrypt_file_to_file(
                        self.config.as_str(),
                        save.as_str(),
                        password,
                    )
                    .await?;
                    exit(0);
                }
                // 解密模式：内存中解密并返回明文供机器人启动
                Mode::Decrypt { password } => {
                    crate::crypto::decrypt_file_to_string(self.config.as_str(), password.as_str())
                        .await
                }
            },
        }
    }
}

