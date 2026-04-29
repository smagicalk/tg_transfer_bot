// file_cache 数据访问入口：
// - `status`：下载中、已就绪、失败等状态回填
// - `refs`：文件引用计数增加/释放
// - `gc`：延迟删除队列扫描、认领和结果回写

mod gc;
mod refs;
mod status;

#[cfg(test)]
pub(in crate::tgbot::transfer) use refs::{acquire_file_ref, release_job_file_refs};
pub(super) use refs::{
    release_file_ref_counts_on_conn, release_job_file_refs_on_conn, try_acquire_file_ref_on_conn,
};

pub(in crate::tgbot::transfer) use gc::{
    claim_file_cache_for_delete, delete_file_cache, list_due_file_cache,
    mark_file_cache_delete_failed,
};
pub(in crate::tgbot::transfer) use status::{
    mark_file_cache_downloading, mark_file_cache_failed, mark_file_cache_ready,
};
