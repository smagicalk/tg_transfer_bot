# tg_transfer_bot 会话恢复文档

本文用于在另一台电脑或新会话中快速恢复 `tg_transfer_bot` 项目上下文。

推荐恢复方式：

```text
请先阅读 AGENTS.md、README.md 和 session.md，然后继续 tg_transfer_bot 项目。
```

## 最新状态

记录日期：2026-05-18

当前开发分支：

```text
dev
```

最近已提交：

```text
7a7656d feat: 优化恢复摘要和卡片交互
7cc04a3 feat: 优化转存卡片交互
fafa07e feat: 优化 downloads/job 返回列表交互
6739982 feat: 优化帮助命令导航
3563be9 style: 格式化 tdlib 生成代码
```

本轮最新提交 `7a7656d` 包含：

- 统一转存卡片字段样式，新增 `card::field`、`card::field_pair`、`card::note`、`card::summary_line`。
- 优化进度面板、下载列表、任务详情、结果卡片和状态卡片展示。
- 进度面板按任务状态显示交互按钮：运行中可暂停/停止，暂停可恢复/停止，已停止只保留详情和定位入口。
- 启动恢复流程增加按请求 chat 聚合的“启动恢复摘要”，可直接进入运行、暂停、停止、全部列表。
- 修正 `workflow` 直接依赖 `command::common` 的模块边界，改为 `command.rs` 受控包装函数。
- 补充进度面板、恢复摘要、callback 路由等边界测试。

提交前已验证：

```powershell
cargo fmt --all
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo check -p transfer_bot
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo test -p transfer_bot -- --nocapture
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
git diff --check
```

验证结果：

```text
cargo test -p transfer_bot: 113 passed
```

当前额外处理：

- 用户要求删除 `docs` 文件夹及其文件。
- 原 `docs/SESSION_HANDOFF.md` 的交接内容已迁移到根目录 `session.md`。

## 项目目标

`tg_transfer_bot` 是一个基于 TDLib 的 Telegram 转存机器人。核心目标是接收转存命令，解析命令里的 Telegram 消息或相册链接，下载对应媒体，再上传到配置的目标 chat。

当前设计重点：

- 文件由 TDLib 管理，本项目只记录文件缓存状态、引用计数和延迟删除计划。
- 同一文件跨任务去重下载，避免重复下载同一个 `file_key`。
- 同一 `source_link + target_chat_id` 的成功任务可复用结果链接。
- 同一 `source_link + target_chat_id` 的活跃任务不重复创建。
- 同一 `request_chat_id + request_message_id` 用于 TDLib 或网络重复投递时的请求级幂等。
- 任务支持启动恢复、下载进度查询、暂停、恢复、停止和延迟清理文件。

## 重要目录

```text
transfer_bot/                       主程序
transfer_bot/src/db/                SeaORM 实体
transfer_bot/src/tgbot/             Telegram/TDLib 交互逻辑
transfer_bot/src/tgbot/transfer/    转存核心逻辑
migration/                          SeaORM 迁移
tdlib_rs/                           TDLib Rust 绑定，批量生成代码，通常不要手动改
```

核心模块：

```text
transfer_bot/src/tgbot/transfer.rs
transfer_bot/src/tgbot/transfer/command/
transfer_bot/src/tgbot/transfer/file.rs
transfer_bot/src/tgbot/transfer/progress.rs
transfer_bot/src/tgbot/transfer/spawn.rs
transfer_bot/src/tgbot/transfer/store.rs
transfer_bot/src/tgbot/transfer/workflow.rs
```

## 数据库表

`transfer_job`：一次转存任务主记录。

关键字段：

- `request_chat_id/request_message_id`：用户发给机器人的命令消息，用于请求级幂等。
- `source_link`：用户输入的源链接。
- `source_chat_id/source_message_id/source_album_id`：真正爬取的源消息信息。
- `target_chat_id`：转存目标 chat。
- `result_message_id/result_message_link`：上传成功后的目标消息入口。
- `status`：任务状态。

`transfer_item`：任务内每条源消息的处理记录。

关键字段：

- `job_id`：所属任务。
- `source_chat_id/source_message_id`：源消息定位。
- `file_key`：稳定文件键，优先 TDLib `remote.unique_id`。
- `status`：子项状态。
- `file_ref_released`：该 item 持有的文件引用是否已经释放，用于防止重复扣减 `file_cache.active_refs`。

`file_cache`：跨任务文件缓存和引用计数。

关键字段：

- `file_key`：文件缓存主键。
- `status`：`pending/downloading/ready/failed/deleting/delete_failed` 等。
- `td_file_id/local_path/size_bytes`：TDLib 文件信息。
- `active_refs`：当前活跃引用数。
- `delete_after`：引用归零后的延迟删除时间。

## 任务状态

主任务状态：

```text
pending
running
paused
cancelling
cancel_finalizing
cancelled
success
failed
partial
```

子项状态：

```text
pending
preparing
prepared
uploading
success
failed
cancelled
obsolete
```

`obsolete` 表示恢复重新 spider 后，旧 item 对应的源消息已经不在当前链接结果里。该 item 不再参与后续下载或上传，并且文件引用会提前释放。

## 核心执行流程

`/transfer` 大致流程：

```text
命令解析
创建进度消息
后台派发 spawn_transfer_job
workflow::transfer
build_transfer_start
复用历史成功结果 / 命中活跃任务 / 请求级幂等 / 创建新任务
run_job_inner
准备下载和 InputMessageContent
全部准备成功后上传
写入 result_message_id/result_message_link
释放文件引用，进入延迟删除队列
编辑进度消息为最终结果
```

查重语义：

- 业务查重：`source_link + target_chat_id`
- 请求幂等：`request_chat_id + request_message_id`

这两层不要混用。请求幂等用于防网络波动重复投递；业务查重用于重复转存时返回已有结果或正在运行的任务。

## 恢复流程

启动入口：

```text
transfer.rs::on_client_ready
workflow::recover_unfinished_jobs
```

启动恢复规则：

- `cancelling/cancel_finalizing`：启动时收敛为 `cancelled` 并释放引用。
- `pending/running`：派发后台恢复任务。
- `paused`：不会自动恢复，必须手动 `/j r <job_id>`。
- 已完成状态不恢复。
- 启动后会按请求 chat 发送恢复摘要，方便直接查看运行、暂停、停止和全部任务列表。

单任务恢复：

```text
spawn_recovery_job
workflow::resume_one_job
acquire_job_guard
apply_job_control
spider_message(source_link)
reconcile_items_for_bundle
mark_job_running
run_job_inner
```

恢复对齐规则：

- 新 spider 多出的消息：新增 `transfer_item`，增加新 `file_key` 引用。
- 同一源消息但 `file_key` 变化：释放旧文件引用，引用新文件，重置 item 为 `pending`。
- 新 spider 缺少的旧消息：标记为 `obsolete`，释放旧文件引用。
- 已提前释放引用的 item 设置 `file_ref_released = true`，最终完成或取消时不会再次扣引用。

## 文件缓存和删除

下载阶段：

- `extract_file_key` 从消息提取稳定文件键。
- `extract_download_seed` 提取 TDLib file id 和大小。
- `run_singleflight(file_key, ...)` 避免同一进程内重复下载同一文件。
- `mark_file_cache_downloading/ready/failed` 只更新缓存状态，不改变引用计数。

引用计数：

- 创建或恢复新增媒体 item 时增加 `file_cache.active_refs`。
- 任务完成、失败、取消时释放本任务未释放的文件引用。
- `active_refs` 归零后设置 `delete_after`，由 GC 循环按配置延迟删除。

删除循环：

```text
workflow::run_file_gc_loop
list_due_file_cache
claim_file_cache_for_delete
tdlib delete_file / 本地路径清理
delete_file_cache 或 mark_file_cache_delete_failed
```

## 上传规则

上传入口：

```text
workflow/upload.rs::upload_prepared
```

规则：

- 单条消息使用 `send_message`。
- 多条消息使用 `send_message_album`，每批最多 10 条。
- album 前会调用 `validate_album_kinds` 做组合校验。
- 文本和语音不能进入 album。
- document album 必须全部是 document。
- audio album 必须全部是 audio。
- photo/video 可以混合 album。

注意：如果目标消息已经实际上传成功，但进程在写入数据库前崩溃，恢复后仍可能重复上传。这是上传阶段天然的幂等风险。

## 回复和交互

回复已经改为卡片式 `FormattedText`：

- `card.rs` 输出卡片标记文本。
- `send/message/content.rs` 将卡片标记转换为 TDLib 原生实体。
- `‹...›` 会变成 code 实体。
- `【文本】(url)` 会变成原生文本链接。
- 第一行标题和 `■` 分区标题会加粗。

结果链接规则：

- 只有 HTTP(S) 链接才显示“打开转存消息”。
- `tg://openmessage` 和纯 `chat_id/message_id` 定位只展示为可复制定位，避免 Telegram 客户端点了不跳转。

进度面板：

- `/transfer` 接收后先回复一条进度消息。
- 后台任务更新时周期性 `editMessageText` 原地刷新。
- 如果 TDLib 初始返回临时 `message_id` 导致 `Message not found`，发送层会等待最终 `message_id` 后重试。
- 运行中可直接点暂停/停止；暂停可直接点恢复/停止；停止态只保留详情和定位。

## 命令概览

短命令和长命令都支持，具体以代码中的 help 为准。

常用命令：

```text
/t <link> [target_chat_id]
/transfer <link> [target_chat_id]

/d [filter] [limit] [page]
/downloads [filter] [limit] [page]

/j p <job_id>
/j r <job_id>
/j s <job_id>
/j st <job_id>

/lookup <link> [target_chat_id]
/lk <link> [target_chat_id]

/config show
/cfg show

/help
/h
```

`/downloads` 常用 filter：

```text
all
wait
dl
up
done
ok
fail
run
ready
pause
cancelling
cancel
```

## 运行和验证

TDLib 路径通常需要先设置：

```powershell
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'
```

常用检查：

```powershell
cargo fmt --all
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo check -p transfer_bot
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo test -p transfer_bot -- --nocapture
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'; cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
cargo test -p migration
```

运行：

```powershell
$env:LOCAL_TDLIB_PATH='F:/tdlib/td/tdlib'
cargo run -p transfer_bot -- -c config.json
```

## 配置和本地状态

不要提交真实配置：

```text
config.json
tg/
*.sqlite
logs/
*.log
```

仓库提供：

```text
config.example.json
```

动态可调配置主要在 `transfer_config`：

- `job_concurrency`
- `file_delete_delay_minutes`
- `file_gc_interval_seconds`

TDLib 登录、API 密钥等不建议通过命令动态修改。

## 已知风险和后续关注

- 上传阶段缺少严格幂等：上传成功但数据库未写入时崩溃，恢复后可能重复上传。
- 恢复对齐以重新 spider 的结果为准，符合当前用户偏好；如果未来需要“冻结首次抓取内容”，需要改成另一套策略。
- `/downloads` 当前已支持按钮翻页和筛选，但后续仍可以继续增强更复杂的按钮交互。
- `tdlib_rs` 是生成代码，不应手动修改。
- 继续做 schema 变更时必须新增 migration，并同步更新 SeaORM 实体和测试 fixture。

## 开发偏好

- 回复、注释、文档默认使用简体中文。
- 代码新增逻辑需要写清楚注释，尤其是并发、恢复、引用计数和状态机相关代码。
- 不为了拆分而拆分，优先保持模块职责清晰。
- 提交信息第一行要能在 GitHub 列表中看懂，正文使用中英双语说明。
- 提交前尽量运行 `fmt`、`check`、`test`、`clippy`。
- `tdlib_rs` 不要动，除非明确要重新生成绑定。
