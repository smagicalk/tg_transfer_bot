// `/help` 的按钮布局。
// help 体系现在只负责页面导航和说明，不再在按钮区重复分发命令复制入口。

use super::super::common::{build_refresh_return_menu_row, build_return_menu_row};
use super::super::menu::build_menu_home_callback_data;
use super::super::{
    cache::build_cache_help_entry_rows, config_cmd::build_config_help_entry_rows,
    downloads::build_downloads_help_entry_rows, health::build_health_help_entry_rows,
    job::build_job_help_entry_rows, lookup::build_lookup_help_entry_rows,
    targets::build_targets_help_entry_rows, transfer_cmd::build_transfer_help_entry_rows,
};
use super::topic::{
    HelpTopicButtonSpec, RuntimeAdminHelpTopic, help_index_admin_button_rows,
    help_index_primary_button_rows, normalize_help_topic, runtime_admin_help_topic,
};
use crate::tgbot::send;

/// `/help` 按钮回调前缀。
const HELP_CALLBACK_PREFIX: &str = "h:";

/// 判断 callback payload 是否属于 `/help`。
pub(super) fn is_help_callback_data(data: &str) -> bool {
    data.starts_with(HELP_CALLBACK_PREFIX)
}

/// 生成 help 页面切换按钮的 callback payload。
pub(super) fn build_help_callback_data(topic: Option<&str>) -> String {
    format!("{}{}", HELP_CALLBACK_PREFIX, topic.unwrap_or("index"))
}

/// 解析 help callback payload。
pub(super) fn parse_help_callback_data(data: &str) -> Option<Option<&str>> {
    let topic = data.strip_prefix(HELP_CALLBACK_PREFIX)?;
    match topic {
        "" | "index" => Some(None),
        other => normalize_help_topic(other).ok().map(Some),
    }
}

/// help 目录页按钮。
pub(super) fn build_help_index_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = build_help_topic_navigation_rows();

    rows.push(build_refresh_return_menu_row(
        send::build_callback_button(
            "刷新",
            &build_help_callback_data(None),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        help_nav_button("帮助说明", "help", tdlib_rs::enums::ButtonStyle::Default),
        menu_home_button(),
    ));
    rows
}

/// help 目录页的 topic 导航区。
///
/// 这个 helper 让 `/help` 和 `/menu` 的帮助入口共享同一份 topic 元数据，
/// 避免两处各自维护 topic 名称、顺序和按钮样式。
pub(super) fn build_help_topic_navigation_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>
{
    let mut rows = vec![
        build_help_topic_button_row(help_index_primary_button_rows()[0]),
        build_help_topic_button_row(help_index_primary_button_rows()[1]),
    ];

    rows.extend(
        help_index_admin_button_rows()
            .iter()
            .map(|row| build_help_topic_button_row(row)),
    );

    rows
}

/// 详细帮助页按钮。
pub(super) fn build_help_detail_buttons(
    command_name: &str,
) -> anyhow::Result<Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>> {
    let command_name = normalize_help_topic(command_name)?;
    if let Some(topic) = runtime_admin_help_topic(command_name) {
        return Ok(build_runtime_admin_help_detail_buttons(topic));
    }
    let rows = match command_name {
        "help" => build_help_entry_footer_rows(vec![]),
        "transfer" => build_help_entry_footer_rows(build_transfer_help_entry_rows()),
        "lookup" => build_help_entry_footer_rows(build_lookup_help_entry_rows()),
        "health" => build_help_entry_footer_rows(build_health_help_entry_rows()),
        "cache" => build_help_entry_footer_rows(build_cache_help_entry_rows()),
        "downloads" => build_help_entry_footer_rows(build_downloads_help_entry_rows()),
        "job" => build_help_entry_footer_rows(build_job_help_entry_rows()),
        "menu" => vec![build_return_menu_row(
            help_index_button(),
            menu_home_button(),
        )],
        _ => anyhow::bail!("unknown help topic: {}", command_name),
    };
    Ok(rows)
}

/// 构造帮助详情页的通用拼装结构。
///
/// 大多数详情页都遵循：
/// - 若干入口按钮
/// - 统一返回 footer
fn build_help_entry_footer_rows(
    entry_rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = entry_rows;
    rows.extend(build_runtime_admin_detail_footer());
    rows
}

/// 构造运行态管理类 topic 的 help 详情按钮。
fn build_runtime_admin_help_detail_buttons(
    topic: RuntimeAdminHelpTopic,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let entry_rows = match topic {
        RuntimeAdminHelpTopic::Config => build_config_help_entry_rows(),
        RuntimeAdminHelpTopic::Targets => build_targets_help_entry_rows(),
    };
    build_help_entry_footer_rows(entry_rows)
}

/// 构建 help 页面切换按钮。
fn help_nav_button(
    text: &str,
    topic: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, &build_help_callback_data(Some(topic)), style)
}

/// 构建返回 help 目录按钮。
fn help_index_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "返回目录",
        &build_help_callback_data(None),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构建返回主菜单按钮。
fn menu_home_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "菜单",
        &build_menu_home_callback_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// help 运行态详情页统一 footer。
fn build_runtime_admin_detail_footer() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![build_return_menu_row(
        help_index_button(),
        menu_home_button(),
    )]
}

/// 把 help topic 元数据转换为目录页按钮行。
///
/// 目录按钮的标题、topic 和主次样式统一从 `help/topic.rs` 读取，避免按钮和正文入口漂移。
fn build_help_topic_button_row(
    specs: &[HelpTopicButtonSpec],
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    specs
        .iter()
        .map(|spec| {
            let style = if spec.primary {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            help_nav_button(spec.label, spec.topic, style)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_detail_buttons_put_navigation_on_last_row() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("transfer")?;
        let last = rows.last().expect("last row");

        assert_eq!(last[0].text, "返回目录");
        assert_eq!(last[1].text, "菜单");
        Ok(())
    }

    #[test]
    fn test_help_job_detail_buttons_prefer_input_callbacks() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("job")?;
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "输入详情");
        assert_eq!(rows[0][1].text, "输入暂停");
        assert_eq!(rows[1][0].text, "输入恢复");
        assert_eq!(rows[1][1].text, "输入停止");
        assert!(!labels.contains(&"复制暂停命令"));
        assert!(!labels.contains(&"复制恢复命令"));
        assert!(!labels.contains(&"复制停止命令"));
        assert!(!labels.contains(&"复制详情命令"));
        assert_eq!(rows[2][0].text, "返回目录");
        assert_eq!(rows[2][1].text, "菜单");
        Ok(())
    }

    #[test]
    fn test_help_detail_buttons_keep_downloads_shortcuts_compact() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("downloads")?;
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "全部列表");
        assert_eq!(rows[0][1].text, "运行列表");
        assert_eq!(rows[0][2].text, "失败列表");
        assert!(!labels.contains(&"复制 /downloads"));
        assert!(!labels.contains(&"复制运行列表"));
        assert_eq!(rows[1][0].text, "返回目录");
        assert_eq!(rows[1][1].text, "菜单");
        Ok(())
    }

    #[test]
    fn test_readonly_help_detail_buttons_prefer_callbacks_without_copy() -> anyhow::Result<()> {
        let health = build_help_detail_buttons("health")?;
        let cache = build_help_detail_buttons("cache")?;
        let health_labels = health
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();
        let cache_labels = cache
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(health[0][0].text, "打开健康页");
        assert_eq!(health[0][1].text, "文件缓存");
        assert_eq!(cache[0][0].text, "打开缓存页");
        assert_eq!(cache[0][1].text, "运行健康");
        assert!(health_labels.iter().all(|label| !label.starts_with("复制")));
        assert!(cache_labels.iter().all(|label| !label.starts_with("复制")));
        Ok(())
    }

    #[test]
    fn test_config_help_detail_buttons_cover_runtime_entry_points() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("config")?;
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"打开配置页"));
        assert!(labels.contains(&"并发详情"));
        assert!(labels.contains(&"删除详情"));
        assert!(!labels.contains(&"复制 /config show"));
        assert!(!labels.contains(&"复制 /config reset"));
        assert!(!labels.contains(&"复制并发"));
        Ok(())
    }

    #[test]
    fn test_runtime_admin_help_detail_buttons_link_to_real_pages() -> anyhow::Result<()> {
        for (topic, expected) in [("targets", vec!["打开目标页", "默认目标", "别名列表"])]
        {
            let rows = build_help_detail_buttons(topic)?;
            let labels = rows
                .iter()
                .flatten()
                .map(|button| button.text.as_str())
                .collect::<Vec<_>>();
            for label in expected {
                assert!(
                    labels.contains(&label),
                    "missing runtime admin page button: {label}"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_runtime_admin_help_detail_buttons_drop_copy_buttons() -> anyhow::Result<()> {
        let targets = build_help_detail_buttons("targets")?;

        let target_labels = targets
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();
        assert!(!target_labels.contains(&"复制 /targets show"));
        assert!(!target_labels.contains(&"复制路由"));
        assert!(!target_labels.contains(&"复制别名"));

        Ok(())
    }
}
