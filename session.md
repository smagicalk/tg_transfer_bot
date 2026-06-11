# tg_transfer_bot 会话恢复文档

用于在另一台电脑或新会话中快速恢复 `tg_transfer_bot` 项目上下文。

推荐恢复提示：

```text
请先阅读 README.md 和 session.md，然后继续 tg_transfer_bot 项目。
```

## 当前状态

记录日期：2026-06-11

当前分支：

```text
dev
```

最近已提交：

```text
d012df0 完善 bot 交互转存链路 / Improve bot transfer workflow
90e5f8b polish telegram interaction cards
4149311 feat: improve transfer menu interactions
7a7656d feat: 优化恢复摘要和卡片交互
7cc04a3 feat: 优化转存卡片交互
```

最近一轮结构收口：

- 命令错误提示和权限/自动转存引导已下沉到 `transfer_bot/src/tgbot/error.rs`。
- 交互回调错误卡片发送已下沉到 `transfer_bot/src/tgbot/send/error.rs`。
- `transfer_bot/src/tgbot/transfer/command/common.rs` 只保留命令公共拼装逻辑。
- `help`、`downloads`、`cache`、`menu`、`points`、`config_cmd`、`job` 的 callback 错误卡片统一走同一发送入口。
- 新增发送失败日志，便于排查 callback 错误卡片本身的发送问题。
- `transfer_bot/src/tgbot/send.rs` 现在只保留发送层入口转发，错误卡片排版细节已拆出。
- `/balance` 与 `/points` 已拆成命令入口、流水渲染和 callback 子模块。
- 新增恢复/查重专项测试：成功结果复用、重复活跃任务、同请求取消幂等、启动恢复扫描。
- 最新验证：`cargo fmt --all`、`cargo check -p transfer_bot`、`cargo test -p transfer_bot`、`cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 全部通过，`cargo test -p transfer_bot` 当前 `263 passed`。

当前未提交改动主要集中在：

- `README.md`
- `session.md`
- `Cargo.toml`
- `transfer_bot/Cargo.toml`
- `transfer_bot/src/main.rs`
- `transfer_bot/src/lib.rs`
- `transfer_bot/src/app_context.rs`
- `transfer_bot/src/db.rs`
- `transfer_bot/src/db/menu_input_draft.rs`
- `transfer_bot/src/db/user_account.rs`
- `transfer_bot/src/db/point_ledger.rs`
- `transfer_bot/src/db/tests.rs`
- `transfer_bot/src/tgbot.rs`
- `transfer_bot/src/tgbot/login.rs`
- `transfer_bot/src/tgbot/send/error.rs`
- `transfer_bot/src/tgbot/send/message.rs`
- `transfer_bot/src/tgbot/queue/progress.rs`
- `transfer_bot/src/tgbot/queue/singleflight.rs`
- `transfer_bot/src/tgbot/transfer.rs`
- `transfer_bot/src/tgbot/transfer/runtime.rs`
- `transfer_bot/src/tgbot/transfer/spawn.rs`
- `transfer_bot/src/tgbot/transfer/progress.rs`
- `transfer_bot/src/tgbot/transfer/workflow.rs`
- `transfer_bot/src/tgbot/transfer/workflow/guard.rs`
- `transfer_bot/src/tgbot/transfer/workflow/recovery.rs`
- `transfer_bot/src/tgbot/transfer/workflow/gc.rs`
- `transfer_bot/src/tgbot/transfer/command/transfer_cmd.rs`
- `transfer_bot/src/tgbot/transfer/command/common.rs`
- `transfer_bot/src/tgbot/transfer/command/points.rs`
- `transfer_bot/src/tgbot/transfer/command/points/render.rs`
- `transfer_bot/src/tgbot/transfer/command/points/callback.rs`
- `transfer_bot/src/tgbot/transfer/command/cache/`
- `transfer_bot/src/tgbot/transfer/command/health.rs`
- `transfer_bot/src/tgbot/transfer/command/menu/`
- `transfer_bot/src/tgbot/transfer/command/job/actions.rs`
- `transfer_bot/src/tgbot/transfer/command/job/callback.rs`
- `transfer_bot/src/tgbot/transfer/store/account.rs`

工作区注意：

- `tdlib_rs` 仍有大量生成代码改动，不要手动清理或重排，除非明确要重新生成绑定。
- `config.json` 是本地敏感配置，已忽略，不要提交。
- 当前同时在推进两条线：`AppContext` 架构重构，以及 bot 菜单/交互一致性优化。
- 当前已经加入 admin/普通用户权限隔离和普通用户积分计费。
- 独立 `migration` crate 已移除，启动时直接建表。
- 当前业务库文件已删除；TDLib 的 `tg/user/db`、`tg/bot/db` 仍保留，不能删。

## 当前架构重构目标

这轮工作的主题仍以“架构优先”为主，但当前实现重心已经部分切到交互层一致性优化。

目标：

- 给 `transfer_bot` 建立正式库边界。
- 把分散的共享状态收进 `AppContext`。
- 先打通 `run -> receive -> login -> transfer startup -> spawn -> workflow` 的显式上下文传递。
- 在不改数据库 schema 和业务语义的前提下，为后续生命周期优化、性能优化和测试隔离做准备。

当前阶段范围：

1. 建立 `transfer_bot` 的库边界。
2. 引入 `AppContext`，统一收口关键共享运行状态。
3. 先打通主链和后台链的显式上下文传递。
4. 保留兼容壳，避免一次性改完整个 `transfer` 目录。

## 当前已完成的重构

已完成：

- `transfer_bot/src/lib.rs` 已新增，承接应用装配逻辑。
- `transfer_bot/src/main.rs` 已缩成纯 Tokio runtime 启动器。
- `transfer_bot/src/app_context.rs` 已新增，当前持有：
  - `TransferRuntimeState`
  - `DownloadProgressStore`
  - `InflightDownloadRegistry`
  - `TransferExecutionGuards`
  - `SendCapabilities`
- 发送层、下载进度、singleflight、进程内 guard、转存运行配置已经有第一层上下文包装。
- 主链 `run -> receive -> handle_update -> handle_authorization -> on_clients_ready` 已开始显式传 `AppContext`。
- 后台链 `on_clients_ready -> spawn -> workflow/recovery/gc/progress` 已开始逐步显式接收并使用 `AppContext`。
- `workflow::transfer`、`resume_one_job`、`run_job_inner` 的入口签名也已经接上 `AppContext`，但内部大部分逻辑仍处于兼容过渡态。
- `health` 和 `store/observability` 已开始直接从 `AppContext` 读取运行时配置和活跃任务计数。

还没完成：

- `workflow::transfer` / `resume_one_job` / `runner` 虽然入口已经接上上下文，但内部还没系统性改成上下文驱动。
- `store::progress`、`job actions`、部分菜单/列表读侧位置仍混用兼容壳和上下文。
- 首页已经加入“继续当前输入 / 取消输入”按钮；继续操作只重新发送当前阶段提示，不消费草稿。

## 现在的关键设计

当前业务设计仍保持不变：

- bot 是唯一交互端；用户号只做源读取/下载 fallback。
- admin 可全局查看和控制任务，不扣积分，可使用 user fallback。
- 普通用户只能私聊 bot，只能查看和控制自己的任务，链接源不允许借 user fallback。
- 普通用户转存按 `billing.base_cost_points + billing.item_cost_points * item_count` 扣积分。
- 扣费发生在 spider 成功后、创建 job 前；无效链接不会扣费。
- 积分账本使用 `request_chat_id + request_message_id` 生成幂等键，防止同一命令重复扣费。
- 任务全部失败或用户停止时会全额退款，并通过 `billing_status = charged -> refunded` 保证幂等。
- 部分成功会按失败条目占比退款；只要存在失败且扣过费，至少退 1 分，最多不超过本次扣费。
- 业务数据库不再使用独立 `migration` crate。
- 启动时直接 `db::ensure_runtime_schema(...)` 建当前完整表结构。
- 开发期允许直接删除业务库，程序下次启动会自动重建。
- 重复转存判断固定看 `source_link + target_chat_id`。
- 请求级幂等固定看 `request_chat_id + request_message_id`。
- 文件缓存保留，消息缓存不做独立表。
- 菜单输入草稿持久化在 `menu_input_draft`，程序重启后未完成交互仍可继续。

## 当前交互流程

### 首页

`/menu` 或 `/m` 打开首页。

首页内容：

- 运行摘要：活跃任务、失败任务、待恢复、待删缓存、删失败、最近任务数
- 直达动作：`开始转存`、`快速转存`、`快速查询`
- 状态直达：`运行任务`、`失败任务`、`已暂停`
- 其他入口：`下载列表`、`任务控制`、`转存页`、`查询页`、`运行配置`、`帮助`、`运行健康`、`文件缓存`、`用户流水`
- 最近任务快捷按钮：运行中可直接 `暂停/停止`，暂停态可 `恢复/停止`
- 如果存在未完成输入，首页顶部会显示 `继续输入：<阶段>` 和 `取消输入`
- admin 可点 `用户流水` 后回复 Telegram user_id，直接进入 `/points history <user_id>` 流水卡片

### 转存

`开始转存` 是三步：

```text
1/3 等待源链接
2/3 选择目标
3/3 确认执行
```

第 2 步目标来源：

- 上次目标
- 默认目标
- 配置别名
- Telegram 原生 `选择群组`
- 手动输入 `chat_id / alias / default`

`快速转存` 会优先用默认目标；如果没有默认目标，会自动退回普通选目标流程。

### 下载列表

支持筛选：

```text
all
run
wait
dl
up
ready
done
ok
fail
pause
cancelling
cancel
```

列表页支持：

- 分页：`首页 / 上页 / 下页 / 末页`
- `刷新`
- `复制当前命令`
- `菜单`
- 单任务直控：运行中 `暂停/停止`，暂停态 `恢复/停止`

### 输入流程行为

- 任意输入流程可发 `/cancel`
- reply keyboard 场景也支持 `取消` / `cancel`
- 从菜单点进输入向导时，原消息会被编辑成等待态，只保留 `取消` / `首页`
- 如果用户在输入流程里直接发送新命令，旧草稿会被丢弃
- 如果旧草稿停在原生选群阶段，会自动收起选群键盘
- `job_id` 输入流也会把原任务页编辑成等待态，避免旧按钮继续可点
- 首页“继续输入”只读取草稿并重新发送当前步骤提示，不会消费草稿；真正消费仍发生在用户回复或确认按钮时

## 数据库与本地状态

当前业务数据库方案：

- 默认业务库：`tg/app/transfer.sqlite`
- 启动时自动建表
- 业务库和 TDLib 状态库分离

当前还保留的 TDLib 数据：

- `tg/user/db/db.sqlite`
- `tg/bot/db/db.sqlite`

不要删除：

- `tg/user/db/`
- `tg/bot/db/`
- `tg/user/files/`
- `tg/bot/files/`

核心表：

- `transfer_job`
- `transfer_item`
- `transfer_result_message`
- `file_cache`
- `menu_input_draft`
- `user_account`
- `point_ledger`

新增权限和积分字段：

- `transfer_job.owner_user_id`：任务归属用户；admin 查询时不加 owner 过滤，普通用户查询时强制过滤。
- `transfer_job.allow_user_fallback`：任务创建时是否允许 user 账号作为源 fallback。
- `transfer_job.cost_points` / `charged_points` / `billing_status`：转存成本、实际扣费和计费状态。
- `user_account.points_balance`：普通用户当前余额。
- `point_ledger.idempotency_key`：扣费幂等键。

新增命令：

- `/balance` / `/bal`：查看当前用户积分余额。
- `/balance history [limit] [page]` / `/bal h [limit] [page]`：查看当前用户积分流水。
- `/points show <user_id>` / `/pts s <user_id>`：admin 查看指定用户积分。
- `/points history <user_id> [limit] [page]` / `/pts h <user_id> [limit] [page]`：admin 查看指定用户积分流水。
- `/points add <user_id> <amount> [reason]` / `/pts a ...`：admin 加分。
- `/points sub <user_id> <amount> [reason]` / `/pts sub ...`：admin 扣分。

本轮已修复：

- `/help points` 现在能正常展开，不再是余额卡片里的死链接。
- `/menu -> 帮助` 已加入积分帮助入口。
- admin 请求 chat 判断不再复用合并后的 `admin_ids`，避免管理员在未授权群聊里被放行。
- `change_points` 改成数据库原子更新，避免并发扣费读到同一旧余额后覆盖。
- `ensure_user_account` 改成显式幂等，重复创建或并发首次创建不会因为 `ON CONFLICT DO NOTHING` 行为失败。
- 失败或取消终态已接入自动退款；并发/重复取消不会重复退款。
- 部分成功终态已接入按失败条目占比退款；账本幂等键仍按 job 保证只退一次。
- `/menu` 首页已支持继续当前输入；`peek_current_draft` 不消费草稿。
- 积分流水查询已接入 `/balance history` 和 `/points history`，默认 10 条、最大 50 条，支持 inline 按钮翻页、刷新和返回菜单。
- 命令级错误卡片已按余额不足、目标不可用、缺少目标、源不可访问和参数格式错误分类，普通用户能看到中文原因和下一步。
- admin 首页已新增 `用户流水` 输入向导，输入 user_id 后复用 `/points history`，不复制积分查询逻辑。

## 最新验证

最近一次完整验证：

```powershell
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'
cargo fmt --all
cargo check -p transfer_bot
cargo test -p transfer_bot
cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
```

结果：

- `cargo check` 通过
- `cargo test` 通过，`256 passed`
- `cargo clippy` 通过

注意：

- 运行测试后会重新生成 `transfer_bot/db.test.sqlite`
- 如果要保持工作区里没有业务数据库，需要手动删掉这个测试库

## 运行方式

```powershell
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'
cargo run -p transfer_bot -- -c config.json
```

当前启动顺序已经变成：

1. `main.rs` 创建 Tokio runtime
2. 调用 `transfer_bot::run()`
3. 读取 `config.json`
4. 初始化业务库路径并确保 schema
5. 构建 `AppContext`
6. 创建 bot/user TDLib client
7. 进入 `tgbot::receive(app_context, config)`

## 下一步建议

优先顺序：

1. 继续把 `workflow::transfer`、`resume_one_job`、`runner`、`store::progress` 从兼容壳推进到直接使用 `AppContext`。
2. 继续统一后台失败卡片和命令失败卡片的错误分类文案，减少两套分类逻辑。
3. 继续优化 admin 运营入口，例如按 user_id 查看该用户任务列表或账户摘要。
4. 最后再考虑把 `db` 和更多命令层状态读写收进更清晰的应用边界。

当前计划可以直接按这个顺序继续，不再单独维护 `plan.md`。
