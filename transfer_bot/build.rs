use std::{env, path::PathBuf};

// 构建脚本：
// 1. 读取本地 TDLib 路径（`LOCAL_TDLIB_PATH`）
// 2. 告诉 rustc 头文件路径与链接库路径
// 3. 配置运行时 rpath，避免运行程序时找不到 tdjson 动态库
// 4. 声明触发重新构建的条件
fn main() {
    // 用户需要在环境变量中提供 TDLib 根目录。
    let dir = env::var("LOCAL_TDLIB_PATH")
        .expect("Please set LOCAL_TDLIB_PATH, e.g. F:\\tdlib\\td\\tdlib");
    let root = PathBuf::from(dir);
    let include_dir = root.join("include");
    let bin_dir = root.join("bin");
    let lib_dir = root.join("lib");

    // 暴露头文件目录，供 bindgen / 依赖使用；使用 PathBuf 避免 Linux CI 中出现 Windows 反斜杠路径。
    println!("cargo:include={}", include_dir.display());
    // 链接器搜索路径：Windows 通常需要 bin/lib，Linux/Alpine 安装后主要在 lib。
    println!("cargo:rustc-link-search=native={}", bin_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // 链接 TDLib 的主动态库。
    println!("cargo:rustc-link-lib=dylib=tdjson");

    // Unix 链接器支持 rpath，CI/部署时即使没有手动设置 LD_LIBRARY_PATH 也能找到 libtdjson。
    if env::var("CARGO_CFG_UNIX").is_ok() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../lib");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    // 当脚本本身或环境变量变化时，触发重新执行 build.rs。
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");
}
