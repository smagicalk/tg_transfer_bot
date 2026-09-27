// `/cache` 命令参数和视图定义。

/// `/cache` 页面视图类型枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CacheView {
    /// 概览视图（状态聚合分布与总览计数）
    Summary,
    /// 列表分页视图（明细缓存条目列表）
    Page,
}

impl CacheView {
    /// 获取视图的标识字符串。
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Page => "page",
        }
    }
}

/// `/cache` 命令解析后的参数结构体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CacheArgs {
    /// 当前所展示的缓存视图类型
    pub view: CacheView,
    /// 单页展示数量
    pub limit: u64,
    /// 当前请求页码
    pub page: u64,
}

impl Default for CacheArgs {
    /// 默认展示分页视图，每页 10 条，从第 1 页开始。
    fn default() -> Self {
        Self {
            view: CacheView::Page,
            limit: 10,
            page: 1,
        }
    }
}

/// 解析 `/cache` 命令行参数切片。
///
/// 规则：
/// - `/cache`: 默认分页视图第 1 页，每页 10 条
/// - `/cache summary [limit] [page]`: 概览视图
/// - `/cache page [limit] [page]`: 明细分页视图
/// - `/cache <limit> [page]`: 简写形式，直接指定单页条数和页码
pub(super) fn parse_cache_args(text: &[&str]) -> anyhow::Result<CacheArgs> {
    // 仅提供 `/cache` 时返回默认参数
    if text.len() <= 1 {
        return Ok(CacheArgs::default());
    }

    let mut args = CacheArgs::default();
    match text[1] {
        // "summary" 或 "sum" 切换为概览视图
        "summary" | "sum" => {
            args.view = CacheView::Summary;
        }
        // "page" 或 "list" 切换为分页明细视图
        "page" | "list" => {
            args.view = CacheView::Page;
        }
        // 尝试判断是否直接输入数字作为 limit 简写
        value => {
            if let Ok(limit) = value.parse::<u64>() {
                args.view = CacheView::Page;
                args.limit = limit.max(1);
                if let Some(page) = text.get(2).and_then(|v| v.parse::<u64>().ok()) {
                    args.page = page.max(1);
                }
                return Ok(args);
            }
            anyhow::bail!("unknown cache subcommand: {value}");
        }
    }

    // 提取第 2 个参数作为 limit
    if let Some(limit) = text.get(2).and_then(|v| v.parse::<u64>().ok()) {
        args.limit = limit.max(1);
    }
    // 提取第 3 个参数作为 page
    if let Some(page) = text.get(3).and_then(|v| v.parse::<u64>().ok()) {
        args.page = page.max(1);
    }
    Ok(args)
}
