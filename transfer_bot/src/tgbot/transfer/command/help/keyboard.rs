// `/help` 的按钮布局。
// 详情页主要保留“复制完整示例”和页面切换，避免把短别名再次暴露给用户。

use super::super::common::{
    CommandStyle, balance_command, balance_history_command, build_copy_only_row,
    build_refresh_return_menu_row, build_return_menu_row, build_runtime_admin_help_copy_rows,
    cache_command, downloads_command, health_command, help_command as help_command_text,
    job_command, points_change_command, points_history_command, points_show_command,
};
use super::super::menu::build_menu_home_callback_data;
use super::super::{
    acl::acl_help_descriptor, billing::billing_help_descriptor, config_cmd::config_help_descriptor,
    targets::targets_help_descriptor,
};
use super::topic::normalize_help_topic;
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
pub(super) fn build_help_index_buttons(
    is_admin: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![
        vec![
            help_nav_button("转存", "transfer", tdlib_rs::enums::ButtonStyle::Primary),
            help_nav_button("查询", "lookup", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button(
                "下载列表",
                "downloads",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            help_nav_button("积分账户", "points", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("任务控制", "job", tdlib_rs::enums::ButtonStyle::Default),
            help_nav_button("交互菜单", "menu", tdlib_rs::enums::ButtonStyle::Default),
        ],
        build_refresh_return_menu_row(
            send::build_callback_button(
                "刷新",
                &build_help_callback_data(None),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            help_nav_button("帮助说明", "help", tdlib_rs::enums::ButtonStyle::Default),
            menu_home_button(),
        ),
        build_copy_only_row(send::build_copy_button(
            "复制流水",
            &balance_history_command(10, 1, CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ];

    if is_admin {
        rows.insert(
            3,
            vec![
                help_nav_button("运行健康", "health", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("文件缓存", "cache", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("运行配置", "config", tdlib_rs::enums::ButtonStyle::Default),
            ],
        );
        rows.insert(
            4,
            vec![
                help_nav_button("目标配置", "targets", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("访问控制", "acl", tdlib_rs::enums::ButtonStyle::Default),
                help_nav_button("计费配置", "billing", tdlib_rs::enums::ButtonStyle::Default),
            ],
        );
        rows.push(vec![
            send::build_copy_button(
                "复制 /health",
                &health_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制 /config reset",
                "/config reset",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]);
        rows.push(vec![send::build_copy_button(
            "复制 /config show",
            "/config show",
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
        rows.push(build_copy_only_row(send::build_copy_button(
            "复制 /cache",
            &cache_command(None, None, None, CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )));
    }

    rows
}

/// 详细帮助页按钮。
pub(super) fn build_help_detail_buttons(
    command_name: &str,
    is_admin: bool,
) -> anyhow::Result<Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>> {
    let command_name = normalize_help_topic(command_name)?;
    let rows = match command_name {
        "help" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制 /help",
                &help_command_text(None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "transfer" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制命令",
                "/transfer https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "lookup" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制命令",
                "/lookup https://t.me/c/123/456",
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "points" => {
            let mut rows = vec![
                build_copy_only_row(send::build_copy_button(
                    "复制 /balance",
                    &balance_command(CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Primary,
                )),
                build_copy_only_row(send::build_copy_button(
                    "复制账户流水",
                    &balance_history_command(10, 1, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                )),
            ];
            if is_admin {
                rows.extend([
                    build_copy_only_row(send::build_copy_button(
                        "复制查看余额",
                        &points_show_command(123456789, CommandStyle::Long),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )),
                    build_copy_only_row(send::build_copy_button(
                        "复制用户流水",
                        &points_history_command(123456789, 10, 1, CommandStyle::Long),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )),
                    build_copy_only_row(send::build_copy_button(
                        "复制加分命令",
                        &points_change_command(
                            "add",
                            123456789,
                            10,
                            "admin_adjust",
                            CommandStyle::Long,
                        ),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )),
                    build_copy_only_row(send::build_copy_button(
                        "复制扣分命令",
                        &points_change_command(
                            "sub",
                            123456789,
                            10,
                            "admin_adjust",
                            CommandStyle::Long,
                        ),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )),
                ]);
            }
            rows.push(build_return_menu_row(
                help_index_button(),
                menu_home_button(),
            ));
            rows
        }
        "health" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制 /health",
                &health_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "cache" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制 /cache",
                &cache_command(None, None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_copy_only_row(send::build_copy_button(
                "复制分页命令",
                &cache_command(Some("page"), None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "config" => [
            build_runtime_admin_help_copy_rows(&config_help_descriptor()),
            vec![build_return_menu_row(
                help_index_button(),
                menu_home_button(),
            )],
        ]
        .concat(),
        "targets" => [
            build_runtime_admin_help_copy_rows(&targets_help_descriptor()),
            vec![build_return_menu_row(
                help_index_button(),
                menu_home_button(),
            )],
        ]
        .concat(),
        "acl" => [
            build_runtime_admin_help_copy_rows(&acl_help_descriptor()),
            vec![build_return_menu_row(
                help_index_button(),
                menu_home_button(),
            )],
        ]
        .concat(),
        "billing" => [
            build_runtime_admin_help_copy_rows(&billing_help_descriptor()),
            vec![build_return_menu_row(
                help_index_button(),
                menu_home_button(),
            )],
        ]
        .concat(),
        "downloads" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制 /downloads",
                &downloads_command(None, None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_copy_only_row(send::build_copy_button(
                "复制运行列表",
                &downloads_command(Some("run"), None, None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "job" => vec![
            build_copy_only_row(send::build_copy_button(
                "复制暂停命令",
                &job_command("pause", 123, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )),
            build_copy_only_row(send::build_copy_button(
                "复制恢复命令",
                &job_command("resume", 123, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            )),
            build_copy_only_row(send::build_copy_button(
                "复制停止命令",
                &job_command("stop", 123, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            )),
            build_copy_only_row(send::build_copy_button(
                "复制详情命令",
                &job_command("status", 123, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            )),
            build_return_menu_row(help_index_button(), menu_home_button()),
        ],
        "menu" => vec![build_return_menu_row(
            help_index_button(),
            menu_home_button(),
        )],
        _ => anyhow::bail!("unknown help topic: {}", command_name),
    };
    Ok(rows)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_detail_buttons_put_navigation_on_last_row() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("transfer", false)?;
        let last = rows.last().expect("last row");

        assert_eq!(last[0].text, "返回目录");
        assert_eq!(last[1].text, "菜单");
        Ok(())
    }

    #[test]
    fn test_help_detail_buttons_use_single_copy_rows() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("job", true)?;

        assert_eq!(rows[0][0].text, "复制暂停命令");
        assert_eq!(rows[1][0].text, "复制恢复命令");
        assert_eq!(rows[2][0].text, "复制停止命令");
        assert_eq!(rows[3][0].text, "复制详情命令");
        assert_eq!(rows[4][0].text, "返回目录");
        assert_eq!(rows[4][1].text, "菜单");
        Ok(())
    }

    #[test]
    fn test_config_help_detail_buttons_cover_runtime_entry_points() -> anyhow::Result<()> {
        let rows = build_help_detail_buttons("config", true)?;
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"复制 /config show"));
        assert!(labels.contains(&"复制 /config reset"));
        assert!(labels.contains(&"复制并发"));
        Ok(())
    }
}
