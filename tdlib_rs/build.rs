use std::{env, path::PathBuf};

// TDLib 绑定 crate 自身声明了 `#[link(name = "tdjson")]`，
// 因此链接库搜索路径也必须由本 crate 提供，保证单独测试 tdlib-rs 时可链接。
fn main() {
    // 用户需要在环境变量中提供 TDLib 根目录。
    let dir = env::var("LOCAL_TDLIB_PATH")
        .expect("Please set LOCAL_TDLIB_PATH, e.g. F:\\tdlib\\td\\tdlib");
    let root = PathBuf::from(dir);
    let bin_dir = root.join("bin");
    let lib_dir = root.join("lib");

    // Windows 下 tdjson.lib 通常在 lib，tdjson.dll 通常在 bin；Linux/Alpine 安装后主要在 lib。
    println!("cargo:rustc-link-search=native={}", bin_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=tdjson");

    // Unix 链接器支持 rpath，单独测试 tdlib-rs 时也能定位到本地安装的 libtdjson。
    if env::var("CARGO_CFG_UNIX").is_ok() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../lib");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    // 变更本脚本或 TDLib 路径后重新执行 build.rs。
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");
}
