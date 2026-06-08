use std::env;

// TDLib 绑定 crate 自身声明了 `#[link(name = "tdjson")]`，
// 因此链接库搜索路径也必须由本 crate 提供，保证单独测试 tdlib-rs 时可链接。
fn main() {
    // 用户需要在环境变量中提供 TDLib 根目录。
    let dir = env::var("LOCAL_TDLIB_PATH")
        .expect("Please set LOCAL_TDLIB_PATH, e.g. F:\\tdlib\\td\\tdlib");

    // Windows 下 tdjson.lib 通常在 lib，tdjson.dll 通常在 bin。
    println!("cargo:rustc-link-search=native={dir}\\bin");
    println!("cargo:rustc-link-search=native={dir}\\lib");
    println!("cargo:rustc-link-lib=dylib=tdjson");

    // 变更本脚本或 TDLib 路径后重新执行 build.rs。
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");
}
