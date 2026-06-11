// `/cache` 命令参数和视图定义。

/// `/cache` 页面视图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CacheView {
    Summary,
    Page,
}

impl CacheView {
    /// 视图字符串。
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Page => "page",
        }
    }
}

/// `/cache` 命令参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CacheArgs {
    pub view: CacheView,
    pub limit: u64,
    pub page: u64,
}

impl Default for CacheArgs {
    fn default() -> Self {
        Self {
            view: CacheView::Summary,
            limit: 10,
            page: 1,
        }
    }
}

/// 解析 `/cache` 命令参数。
pub(super) fn parse_cache_args(text: &[&str]) -> anyhow::Result<CacheArgs> {
    if text.len() <= 1 {
        return Ok(CacheArgs::default());
    }

    let mut args = CacheArgs::default();
    match text[1] {
        "summary" | "sum" => {
            args.view = CacheView::Summary;
        }
        "page" | "list" => {
            args.view = CacheView::Page;
        }
        value => {
            if let Ok(limit) = value.parse::<u64>() {
                args.view = CacheView::Page;
                args.limit = limit.max(1);
                if let Some(page) = text.get(2).and_then(|v| v.parse::<u64>().ok()) {
                    args.page = page.max(1);
                }
                return Ok(args);
            }
            anyhow::bail!("unknown cache subcommand: {}", value);
        }
    }

    if let Some(limit) = text.get(2).and_then(|v| v.parse::<u64>().ok()) {
        args.limit = limit.max(1);
    }
    if let Some(page) = text.get(3).and_then(|v| v.parse::<u64>().ok()) {
        args.page = page.max(1);
    }
    Ok(args)
}
