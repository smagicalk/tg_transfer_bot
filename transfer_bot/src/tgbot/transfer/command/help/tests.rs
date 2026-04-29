// `/help` 文案测试。
// 这里覆盖公开命令目录和几个常用 topic 的展开结果。

use super::text::{build_help_detail_text, build_help_index_text};

// help 目录页应包含所有公开命令入口。
#[test]
fn test_build_help_index_text_contains_commands() {
    let text = build_help_index_text();
    assert!(text.contains("/help"));
    assert!(text.contains("/transfer"));
    assert!(text.contains("/lookup"));
    assert!(text.contains("/config"));
    assert!(text.contains("/downloads"));
    assert!(text.contains("/job"));
}

// 详细帮助应能分别展开不同命令。
#[test]
fn test_build_help_detail_text() {
    let transfer = build_help_detail_text("transfer").unwrap();
    assert!(transfer.contains("/transfer <link> [target_chat_id]"));
    let transfer_short = build_help_detail_text("t").unwrap();
    assert!(transfer_short.contains("/transfer <link> [target_chat_id]"));
    let transfer_slash = build_help_detail_text("/t").unwrap();
    assert!(transfer_slash.contains("/transfer <link> [target_chat_id]"));

    let downloads = build_help_detail_text("downloads").unwrap();
    assert!(downloads.contains(
        "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel"
    ));
    let downloads_short = build_help_detail_text("d").unwrap();
    assert!(downloads_short.contains("/downloads [filter] [limit] [page]"));

    let job = build_help_detail_text("j").unwrap();
    assert!(job.contains("/job pause 123"));

    let config = build_help_detail_text("config").unwrap();
    assert!(config.contains("/config set job_concurrency 4"));

    assert!(build_help_detail_text("unknown").is_err());
}
