//! 加解密工具模块。
//!
//! 方案：Argon2 强哈希派生密钥 + AES-256-GCM 认证加密，统一采用 Base64 文本作为序列化格式。
//! 密文封装格式为：`[16 字节 Salt] + [12 字节 Nonce] + [密文数据 + 16 字节认证标签 Auth Tag]`。

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use tokio::fs;

/// 根据用户密码口令与随机盐值，使用 Argon2id 算法安全派生 32 字节（256 位）对称密钥。
///
/// Argon2 计算失败时向上返回错误，避免在配置加解密路径中发生不可控 panic。
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut key)?;
    Ok(key)
}

/// 加密明文字节数组，返回 Base64 编码的密文字符串。
///
/// 内部每次自动生成 16 字节随机 Salt 和 12 字节随机 Nonce，
/// 输出载荷布局：`salt(16) + nonce(12) + ciphertext(带 auth tag)`。
pub fn encrypt_bytes(plaintext: &[u8], password: &str) -> Result<String> {
    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);

    let key_bytes = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;

    let mut nonce = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce);

    let ciphertext = cipher.encrypt(&nonce.into(), plaintext)?;

    let mut out = Vec::new();
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(out))
}

/// 解密 Base64 密文字符串，返回明文字节数组。
///
/// 提取开头的 16 字节 Salt 与 12 字节 Nonce，通过 Argon2 派生出密钥后使用 AES-256-GCM 验证并解密。
pub fn decrypt_bytes(ciphertext_b64: &str, password: &str) -> Result<Vec<u8>> {
    let data = general_purpose::STANDARD.decode(ciphertext_b64)?;
    if data.len() < 16 + 12 {
        anyhow::bail!("密文长度不足");
    }

    let (salt, rest) = data.split_at(16);
    let (nonce_bytes, ciphertext) = rest.split_at(12);

    let key_bytes = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
    let nonce = Nonce::try_from(nonce_bytes)?;
    let plaintext = cipher.decrypt(&nonce, ciphertext)?;
    Ok(plaintext)
}

/// 异步读取指定路径的文件完整内容为字节数组。
pub async fn read_file(path: &str) -> Result<Vec<u8>> {
    Ok(fs::read(path).await?)
}

/// 异步将字节数组写入指定路径的目标文件。
pub async fn write_file(path: &str, data: &[u8]) -> Result<()> {
    fs::write(path, data).await?;
    Ok(())
}

/// 读取输入文件，对其内容进行加密后将 Base64 文本写入输出文件。
pub async fn encrypt_file_to_file(input: &str, output: &str, password: &str) -> Result<()> {
    let data = read_file(input).await?;
    let enc_b64 = encrypt_bytes(&data, password)?;
    write_file(output, enc_b64.as_bytes()).await
}

/// 读取输入文件，对其内容进行加密后直接返回 Base64 字符串。
pub async fn encrypt_file_to_string(input: &str, password: &str) -> Result<String> {
    let data = read_file(input).await?;
    encrypt_bytes(&data, password)
}

/// 将明文字符串加密为 Base64 文本并写入输出文件。
pub async fn encrypt_string_to_file(plaintext: &str, output: &str, password: &str) -> Result<()> {
    let enc_b64 = encrypt_bytes(plaintext.as_bytes(), password)?;
    write_file(output, enc_b64.as_bytes()).await
}

/// 读取 Base64 密文文件并将其解密写入目标明文文件。
pub async fn decrypt_file_to_file(input: &str, output: &str, password: &str) -> Result<()> {
    let enc_b64 = String::from_utf8(read_file(input).await?)?;
    let plain = decrypt_bytes(&enc_b64, password)?;
    write_file(output, &plain).await
}

/// 读取 Base64 密文文件并解密还原为 UTF-8 明文字符串。
pub async fn decrypt_file_to_string(input: &str, password: &str) -> Result<String> {
    let enc_b64 = String::from_utf8(read_file(input).await?)?;
    let plain = decrypt_bytes(&enc_b64, password)?;
    Ok(String::from_utf8(plain)?)
}

/// 直接解密 Base64 密文字符串并将还原后的明文字节写入输出文件。
pub async fn decrypt_string_to_file(
    ciphertext_b64: &str,
    output: &str,
    password: &str,
) -> Result<()> {
    let plain = decrypt_bytes(ciphertext_b64, password)?;
    write_file(output, &plain).await
}

// 基础测试：覆盖字符串/文件的往返加解密流程。
#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngExt;
    use std::path::{Path, PathBuf};

    // 测试临时目录：把测试文件限制在系统临时目录里，避免污染项目目录。
    struct CryptoTestDir {
        path: PathBuf,
    }

    impl CryptoTestDir {
        // 使用随机后缀隔离并行测试，避免多个测试读写同名文件。
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "tg_transfer_bot_crypto_{}_{}",
                name,
                rand::rng().random::<u64>()
            ));
            std::fs::create_dir_all(&path).expect("create crypto test temp dir");
            Self { path }
        }

        // 拼出当前测试目录下的文件路径。
        fn file(&self, name: &str) -> PathBuf {
            self.path.join(name)
        }
    }

    impl Drop for CryptoTestDir {
        fn drop(&mut self) {
            // 测试结束后清理临时目录；失败时忽略，避免掩盖真正的断言错误。
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    // 现有加解密 API 使用 &str 路径，这里统一做 Path -> String 转换。
    fn path_string(path: &Path) -> String {
        path.to_str()
            .expect("test path must be valid UTF-8")
            .to_owned()
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_string_to_file() {
        let password = "my_password";
        let text = "Hello Rust!";
        let dir = CryptoTestDir::new("string_to_file");
        let enc_path = path_string(&dir.file("enc.txt"));
        let out_path = path_string(&dir.file("out.txt"));

        encrypt_string_to_file(text, &enc_path, password)
            .await
            .unwrap();
        decrypt_file_to_file(&enc_path, &out_path, password)
            .await
            .unwrap();

        let recovered = tokio::fs::read_to_string(out_path).await.unwrap();
        assert_eq!(recovered, text);
    }

    #[tokio::test]
    async fn test_file_to_string_and_back() {
        let password = "my_password";
        let dir = CryptoTestDir::new("file_to_string_and_back");
        let plain_path = path_string(&dir.file("plain.txt"));
        let out_path = path_string(&dir.file("out.txt"));

        tokio::fs::write(&plain_path, "File content").await.unwrap();

        let b64 = encrypt_file_to_string(&plain_path, password).await.unwrap();
        decrypt_string_to_file(&b64, &out_path, password)
            .await
            .unwrap();

        let recovered = tokio::fs::read_to_string(out_path).await.unwrap();
        assert_eq!(recovered, "File content");
    }

    #[tokio::test]
    async fn test_file_to_string_direct() {
        let password = "my_password";
        let dir = CryptoTestDir::new("file_to_string_direct");
        let plain_path = path_string(&dir.file("plain.txt"));

        tokio::fs::write(&plain_path, "ABC123").await.unwrap();

        let b64 = encrypt_file_to_string(&plain_path, password).await.unwrap();
        let plain = decrypt_bytes(&b64, password).unwrap();

        assert_eq!(plain, b"ABC123");
    }
}
