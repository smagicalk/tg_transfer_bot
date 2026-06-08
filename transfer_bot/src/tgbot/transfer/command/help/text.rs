// `/help` 文本文案入口。
// 这里只负责聚合目录页和详情页，具体长文案放到子文件中维护。

mod detail;
mod index;

pub(super) use detail::build_help_detail_text;
pub(super) use index::build_help_index_text;
