use std::env;

// 构建脚本：
// 1. 读取本地 TDLib 路径（`LOCAL_TDLIB_PATH`）
// 2. 告诉 rustc 头文件路径与链接库路径
// 3. 配置运行时 rpath，避免运行程序时找不到 tdjson 动态库
// 4. 声明触发重新构建的条件
fn main() {
    // 用户需要在环境变量中提供 TDLib 根目录。
    let dir = env::var("LOCAL_TDLIB_PATH")
        .expect("Please set LOCAL_TDLIB_PATH, e.g. F:\\tdlib\\td\\tdlib");

    // 暴露头文件目录，供 bindgen / 依赖使用。
    println!("cargo:include={dir}\\include");
    // 链接器搜索路径：Windows 下一般在 bin / lib。
    println!("cargo:rustc-link-search=native={dir}\\bin");
    println!("cargo:rustc-link-search=native={dir}\\lib");
    // 链接 TDLib 的主动态库。
    println!("cargo:rustc-link-lib=dylib=tdjson");

    // 运行时库搜索路径（不同平台/加载器行为不同，保留多条兼容规则）。
    println!("cargo:rustc-link-arg=-Wl,-rpath,{dir}\\bin");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/bin");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    // 当脚本本身或环境变量变化时，触发重新执行 build.rs。
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");
}
