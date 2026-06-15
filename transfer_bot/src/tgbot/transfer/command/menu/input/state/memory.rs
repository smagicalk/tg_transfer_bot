// `/menu` 输入状态里的纯内存辅助能力。
// 这里只放不会落库的交互优化和进程内互斥，避免和数据库草稿持久化逻辑混在一起。

use std::collections::{HashMap, HashSet};
use std::sync::MutexGuard;

use super::{DraftKey, MENU_DRAFT_ACTIVE_KEYS, MENU_LAST_TARGETS};

/// 记录用户最近一次确认执行的目标 chat。
pub(in crate::tgbot::transfer::command::menu) fn remember_last_target(
    chat_id: i64,
    user_id: i64,
    target_chat_id: i64,
) {
    let mut targets = lock_menu_last_targets();
    targets.insert((chat_id, user_id), target_chat_id);
    tracing::debug!(
        chat_id,
        user_id,
        target_chat_id,
        "menu last target remembered"
    );
}

/// 读取用户最近一次确认执行的目标 chat。
pub(in crate::tgbot::transfer::command::menu) fn last_target(
    chat_id: i64,
    user_id: i64,
) -> Option<i64> {
    let targets = lock_menu_last_targets();
    targets.get(&(chat_id, user_id)).copied()
}

/// 获取最近目标锁；锁中毒时恢复内部 HashMap，避免交互缓存故障扩散成菜单不可用。
fn lock_menu_last_targets() -> MutexGuard<'static, HashMap<DraftKey, i64>> {
    match MENU_LAST_TARGETS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned menu last target mutex");
            poisoned.into_inner()
        }
    }
}

/// 获取某个草稿键的进程内互斥。
///
/// 锁表只保存正在处理的 key，不在 await 期间持有 `MutexGuard`，因此不会阻塞其它用户的输入。
pub(in crate::tgbot::transfer::command::menu) async fn acquire_draft_key_guard(
    key: DraftKey,
) -> MenuDraftKeyGuard {
    loop {
        {
            let mut keys = lock_menu_draft_active_keys();
            if keys.insert(key) {
                return MenuDraftKeyGuard { key };
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}

/// 草稿键互斥 guard。
pub(in crate::tgbot::transfer::command::menu) struct MenuDraftKeyGuard {
    key: DraftKey,
}

impl Drop for MenuDraftKeyGuard {
    fn drop(&mut self) {
        let mut keys = lock_menu_draft_active_keys();
        keys.remove(&self.key);
    }
}

/// 获取草稿互斥锁；锁中毒时恢复集合，避免单个 panic 让所有菜单输入不可用。
fn lock_menu_draft_active_keys() -> MutexGuard<'static, HashSet<DraftKey>> {
    match MENU_DRAFT_ACTIVE_KEYS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned menu draft key mutex");
            poisoned.into_inner()
        }
    }
}
