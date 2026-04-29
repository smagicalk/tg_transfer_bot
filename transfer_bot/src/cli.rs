// 命令行参数模块：
// 1. 指定配置文件路径
// 2. 可选执行配置文件加密/解密

use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(name = "cli", version, about = "转存机器人命令行入口")]
pub(crate) struct TransferBotCli {
    // 配置文件路径（必填）。
    #[arg(
        short = 'c',
        long = "config",
        required = true,
        help = "配置文件路径（必填）"
    )]
    pub config: String,

    // 可选模式：Encrypt / Decrypt / None。
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    // 默认模式：不加密不解密，仅读取配置。
    #[command(about = "默认模式（不加密/不解密）", hide = true)]
    None,

    // 加密配置文件：输出 `<config>.enc`。
    #[command(about = "加密并指定密码", visible_alias = "enc")]
    Encrypt { password: String },

    // 解密配置文件：读取密文，返回明文字符串。
    #[command(about = "解密并指定密码", visible_alias = "dec")]
    Decrypt { password: String },
}

impl TransferBotCli {
    // 异步读取配置文件文本。
    async fn read_file(path: &std::path::PathBuf) -> anyhow::Result<String> {
        if !path.exists() {
            anyhow::bail!("{:?} 文件不存在", path)
        }
        if path.is_dir() {
            anyhow::bail!("{:?} 是目录，不是文件", path)
        }
        Ok(tokio::fs::read_to_string(path).await?)
    }

    // 统一获取配置内容：
    // - None / Mode::None：直接读取配置
    // - Encrypt：加密后退出进程
    // - Decrypt：返回解密明文
    pub async fn get_config(&self) -> anyhow::Result<String> {
        let path = std::path::PathBuf::from(&self.config);
        match &self.mode {
            None => Self::read_file(&path).await,
            Some(mode) => match mode {
                Mode::None => Self::read_file(&path).await,
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
                Mode::Decrypt { password } => {
                    crate::crypto::decrypt_file_to_string(self.config.as_str(), password.as_str())
                        .await
                }
            },
        }
    }
}
