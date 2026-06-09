# tg_transfer_bot

`tg_transfer_bot` 是一个基于 TDLib 的 Telegram 转存机器人，用于把指定消息链接或相册链接下载到本地，再上传到配置的目标聊天。项目的核心目标是减少手动保存文件的工作量，并尽量保证转存任务可恢复、可查询、可控制。

## 当前能力

- 支持 `/transfer` 转存单条消息或相册。
- 支持相同文件下载去重，多个任务引用同一文件时不会重复下载。
- 支持文件引用计数，引用归零后进入延迟删除队列。
- 支持任务进度消息，转存开始后会回复一条进度消息并持续编辑更新。
- 支持 `/downloads` 查询任务列表、状态、分页和真实下载进度。
- 支持 `/job` 手动暂停、恢复、停止任务。
- 支持 `/lookup` 按源链接查询已经成功转存的目标链接。
- 支持 `/config` 动态调整部分运行参数，并写回本地配置。
- 程序启动时会从数据库恢复未完成任务。

## 项目结构

```text
.
├── config.example.json     # 本地配置模板，不包含真实密钥
├── migration/              # SeaORM 数据库迁移
├── tdlib_rs/               # TDLib Rust 绑定，批量生成代码
└── transfer_bot/           # 机器人主程序
```

`tdlib_rs` 是生成代码，正常业务开发时不需要手动改它。主要业务逻辑在 `transfer_bot/src/tgbot/transfer/`。

## 运行前准备

### 1. 准备 TDLib

项目通过 `tdjson` 连接 TDLib。构建和运行前需要设置 `LOCAL_TDLIB_PATH`，指向本机 TDLib 安装目录。

PowerShell 示例：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
```

该目录通常需要包含：

- `include/`
- `bin/tdjson.dll`
- `lib/`

### 2. 准备配置文件

以 [config.example.json](config.example.json) 为模板创建本地 `config.json`。真实配置文件已被 `.gitignore` 忽略，不要提交到仓库。

关键字段说明：

| 字段 | 说明 |
| --- | --- |
| `config_version` | 配置版本，目前为 `2` |
| `tdlib_defaults.api_id` | Telegram API ID，user/bot 共用 |
| `tdlib_defaults.api_hash` | Telegram API Hash，user/bot 共用 |
| `storage.database_url` | 机器人业务 SQLite 数据库连接串，保存转存任务、文件引用和恢复状态 |
| `clients.user.login_info` | 用户号登录方式，支持 `OCR`、`PHONE` |
| `clients.user.tdlib.database_directory` | 用户号 TDLib 数据库目录，当前建议 `tg/user/db` |
| `clients.user.tdlib.files_directory` | 用户号 TDLib 文件目录，当前建议 `tg/user/files` |
| `clients.*.tdlib.database_encryption_key` | TDLib 数据库加密 key，配置里填普通字符串；程序发给 TDLib 前会自动转成 JSON bytes 要求的 base64 |
| `clients.bot.enabled` | 是否启用 bot client |
| `clients.bot.token` | BotFather 生成的 bot token，格式应类似 `<数字 bot id>:<token secret>` |
| `clients.bot.tdlib.database_directory` | bot TDLib 数据库目录，当前建议 `tg/bot/db` |
| `clients.bot.tdlib.files_directory` | bot TDLib 文件目录，当前建议 `tg/bot/files` |
| `workflow.interaction_client` | 固定为 `bot`，命令、卡片按钮、callback 和 copy-text 都只能由 bot 交互端处理 |
| `workflow.download_client` | 兼容字段，建议填 `bot`；真实源读取/下载按 bot-first、user fallback 策略自动决定 |
| `workflow.upload_client` | 谁上传到目标 chat，可选 `user` 或 `bot` |
| `access_control.admin_user_ids` | 允许管理机器人的用户 id |
| `access_control.allowed_request_chat_ids` | 允许发命令的 chat id |
| `access_control.allowed_target_chat_ids` | 允许转存到的目标 chat id；空数组表示不限制 |
| `targets.default_chat_id` | 未显式传目标时的兜底目标 chat |
| `targets.by_request_chat_id` | 按请求 chat 映射默认目标 chat |
| `targets.aliases` | 目标 chat 别名，例如命令里用 `archive` 代替数字 chat_id |
| `transfer_config.job_concurrency` | 后台转存任务并发数 |
| `transfer_config.file_delete_delay_minutes` | 文件引用归零后的延迟删除分钟数 |
| `transfer_config.file_gc_interval_seconds` | 文件删除队列扫描间隔秒数 |
| `transfer_config.progress_edit_interval_seconds` | 转存进度消息最短编辑间隔秒数 |
| `transfer_config.downloads_default_page_size` | `/downloads` 默认分页大小 |
| `transfer_config.menu_input_timeout_seconds` | 菜单等待输入的超时时间秒数 |

常见 workflow 组合：

```json
{
  "interaction_client": "bot",
  "download_client": "bot",
  "upload_client": "bot"
}
```

这表示 bot 负责命令交互、优先读取/下载源链接和上传；如果 bot 读不到源链接或 bot 准备文件失败，会切换 user 作为备用源重新读取/下载。`interaction_client` 必须保持 `bot`；如果希望用户号上传，只把 `upload_client` 改成 `user`。重复转存判断仍然只看 `source_link + target_chat_id`，不区分上传者。

`config.json`、`tg/`、SQLite 数据库和日志文件都是本地运行状态，不应该上传到 GitHub。
默认业务数据库路径是 `tg/app/transfer.sqlite`；TDLib 的 user/bot 数据库只保存 Telegram client 状态，不保存转存任务。

## 运行

在仓库根目录运行：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo run -p transfer_bot -- -c config.json
```

### 日志排查

默认日志会同时输出到控制台和仓库根目录的 `tg_transfer.log`。默认级别适合日常运行，只保留启动、登录、命令、任务、上传和清理等主流程。

如果机器人“没有反应”，先打开 debug 级别运行：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
$env:RUST_LOG = "transfer_bot=debug,info,sea_orm=warn,sqlx=warn,tokio=warn"
cargo run -p transfer_bot -- -c config.json
```

需要追踪 TDLib update 或真实文件进度时，再临时打开 trace：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
$env:RUST_LOG = "transfer_bot=trace,info,sea_orm=warn,sqlx=warn,tokio=warn"
cargo run -p transfer_bot -- -c config.json
```

常用日志关键词：

| 关键词 | 含义 |
| --- | --- |
| `tdlib authorization ready` | TDLib 登录成功，session 有效 |
| `starting transfer background services` | 转存后台服务已启动 |
| `admin command received` | 管理员命令已进入命令分发 |
| `ignored historical message` | 忽略启动前的历史消息 |
| `ignored non-admin message` | chat 或 sender 不在访问控制白名单 |
| `ignored non-text admin message` | 管理员发了非文本消息，不能当命令处理 |
| `transfer callback query routed` | 按钮回调已进入对应命令模块 |
| `tdlib receive loop exited with error` | 主接收循环异常退出，需要看 error 字段 |

如果使用加密配置，可以先生成密文：

```powershell
cargo run -p transfer_bot -- -c config.json encrypt <password>
```

再使用密文启动：

```powershell
cargo run -p transfer_bot -- -c config.json.enc decrypt <password>
```

## 机器人命令

命令都提供长写法和短写法。长写法适合阅读，短写法适合日常使用。

| 命令 | 短命令 | 作用 |
| --- | --- | --- |
| `/help [command]` | `/h [command]` | 查看命令目录或具体命令说明 |
| `/transfer <link> [target]` | `/t <link> [target]` | 创建转存任务 |
| `/lookup <link> [target]` | `/lk <link> [target]` | 查询历史转存结果 |
| `/downloads [filter] [limit] [page]` | `/d [filter] [limit] [page]` | 查询任务列表和下载进度 |
| `/job <pause|resume|stop> <job_id>` | `/j <p|r|s> <job_id>` | 手动控制任务 |
| `/config [show|set <key> <value>]` | `/cfg [show|set <key> <value>]` | 查看或修改运行配置 |

### 转存

```text
/t https://t.me/c/123/456
/transfer https://t.me/c/123/456 -1001234567890
```

不传 `target_chat_id` 时，会优先按 `targets.by_request_chat_id[request_chat_id]` 查找目标聊天；找不到时使用 `targets.default_chat_id`。
显式目标可以填数字 chat_id，也可以填 `targets.aliases` 里的别名；如果配置了 `access_control.allowed_target_chat_ids`，所有目标都必须在白名单内。

### 查询历史结果

```text
/lk https://t.me/c/123/456
/lookup https://t.me/c/123/456 -1001234567890
```

`lookup` 只查询已经成功完成的转存结果。如果同一个源链接转存到不同目标聊天，需要带上目标 chat id 区分。

### 查询任务列表

```text
/d
/d 10
/d dl
/d done 5
/d done 5 2
```

`downloads` 支持分页按钮，也支持命令参数分页。可用筛选值：

```text
all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel
```

### 控制任务

```text
/j p 123
/j r 123
/j s 123
```

- `pause` / `p`：暂停任务。
- `resume` / `r`：恢复任务。
- `stop` / `s`：停止任务，释放文件引用，并按延迟删除策略清理文件。

### 动态配置

```text
/cfg show
/cfg set job_concurrency 4
/cfg set file_delete_delay_minutes 3
/cfg set file_gc_interval_seconds 30
/cfg set progress_edit_interval_seconds 3
/cfg set downloads_default_page_size 10
/cfg set menu_input_timeout_seconds 900
```

只有转存运行参数支持动态修改。TDLib 登录、API ID、API Hash 等启动级配置不通过机器人命令修改。

## 任务流程

### 用户视角

1. 管理员发送 `/transfer <link> [target]`。
2. 机器人立即回复一条进度卡片，后续通过编辑消息持续刷新状态。
3. 后台解析源链接，抓取单条消息或完整相册。
4. 任务先写入数据库，避免进程退出后丢失状态。
5. 每条源消息对应一个 `transfer_item`，媒体文件通过 `file_cache` 引用计数和下载去重。
6. 文件准备完成后上传到目标聊天；多条图片/视频等会尽量按 album 上传。
7. 上传完成后保存目标消息 ID 和 TDLib 返回的 HTTP(S) 链接；如果目标 chat 没有可跳转链接，则保存可复制定位信息。
8. 任务完成、失败或停止时释放文件引用；引用为 0 的文件进入延迟删除队列。
9. 程序重启后会恢复数据库中的未完成任务。

### 开发调用流程

下面是从进程启动到完成转存的主调用链，开发时可以按这个顺序阅读代码。

```text
main()
  -> logs::init_tracing()
  -> migration::Migrator::up(...)
  -> config::init_runtime_config_path(...)
  -> tgbot::transfer::init_runtime_config(...)
  -> tgbot::create_client()
  -> tgbot::receive(...)
```

`tgbot::receive` 是 TDLib update 的主循环：

```text
tgbot::receive(config)
  -> tdlib_rs::receive()
  -> tokio::spawn(handle_update(update, config))
```

`handle_update` 根据 update 类型分发：

```text
handle_update(update, config)
  -> AuthorizationState
       -> login::handle_authorization(...)
       -> transfer::on_client_ready(client_id)
  -> MessageSendSucceeded / MessageSendFailed
       -> send::observe_message_send_succeeded(...)
       -> send::observe_message_send_failed(...)
  -> NewMessage
       -> /help       -> transfer::help_command(...)
       -> /transfer   -> transfer::transfer_command(...)
       -> /lookup     -> transfer::lookup_command(...)
       -> /downloads  -> transfer::downloads_command(...)
       -> /job        -> transfer::job_command(...)
       -> /config     -> transfer::config_command(...)
  -> NewCallbackQuery
       -> transfer::downloads_callback_query(...)
  -> File
       -> queue::update_download_progress(...)
```

### `/transfer` 函数流程

`/transfer` 的命令入口只负责解析参数、发进度卡片、派发后台任务：

```text
transfer::command::transfer_cmd::transfer_command(...)
  -> resolve_target_chat_id(...)
  -> send::send_markdown_message_with_buttons_returning(...)
  -> spawn_transfer_job(plan, request_chat_id, progress_message_id, client_id)
```

后台任务负责占用并发槽、刷新进度、执行工作流、写回最终结果：

```text
transfer::spawn::spawn_transfer_job(...)
  -> tokio::spawn(...)
  -> progress::update_transfer_progress_message(...)
  -> runtime::acquire_transfer_slot()
  -> workflow::transfer(plan, client_id)
  -> progress::edit_transfer_progress_for_outcome(...)
  -> outcome::send_transfer_outcome(...)   # 仅在编辑进度消息失败或没有进度消息时发送独立结果卡片
```

`workflow::transfer` 决定“复用、恢复还是新建任务”：

```text
workflow::transfer(plan, client_id)
  -> start::build_transfer_start(plan, client_id)
       -> acquire_source_target_create_guard(source_link, target_chat_id)
       -> store::find_success_job_by_source_target(...)
       -> store::find_active_job_by_source_target(...)
       -> store::find_job_by_request(...)
       -> spider::spider_message(...)
       -> store::create_job(...)
       -> guard::acquire_job_guard(job_id)
       -> store::ensure_items_for_bundle(...)
  -> TransferStart::Outcome(...)  # 直接返回历史结果、运行中、暂停中、停止中等状态
  -> TransferStart::Resume(job)   # 继续旧任务
  -> TransferStart::Run(job, messages, guard)
       -> runner::run_job_inner(job, messages, client_id)
```

`run_job_inner` 是真正的下载、准备、上传状态机：

```text
runner::run_job_inner(job, messages, client_id)
  -> control::apply_job_control(job_id)
  -> store::list_items_by_job(job_id)
  -> 对每条源消息循环：
       -> store::set_item_status(item_id, preparing)
       -> file::extract_download_seed(message)
       -> store::mark_file_cache_downloading(seed)
       -> queue::run_singleflight(file_key, ensure_media_downloaded)
       -> file::prepare_upload_content(message, client_id)
       -> store::mark_file_cache_ready(meta)
       -> store::set_item_status(item_id, prepared)
  -> 准备失败：
       -> store::finish_job_with_item_statuses(...)
       -> release_job_file_refs(...)
  -> 准备成功：
       -> store::set_item_status(item_id, uploading)
       -> upload::upload_prepared(target_chat_id, prepared, client_id)
       -> upload::build_result_message_link(...)
       -> store::replace_result_messages_on_conn(...)
       -> store::finish_uploaded_job_with_item_statuses(...)
       -> release_job_file_refs(...)
```

上传阶段的关键点：

- 单条消息走 `tdlib_rs::functions::send_message`。
- 多条消息走 `tdlib_rs::functions::send_message_album`，每批最多 10 条。
- 超过 10 条会产生多个结果入口，首个入口继续写入 `transfer_job.result_message_*`，全部入口写入 `transfer_result_message`。
- `upload::validate_album_kinds` 会提前拒绝 TDLib 不支持的 album 组合，例如多条纯文本、多条语音、document 与 photo/video 混发。
- 发送后会调用 `send::wait_for_sent_message` 等待 TDLib 把临时 `message_id` 替换成最终 `message_id`。
- `getMessageLink` 只在 TDLib 支持时返回 HTTP(S) 链接；失败时使用 `chat_id=... message_id=...` 作为可复制定位信息。

### 启动恢复流程

客户端 ready 后只启动一次恢复和文件 GC：

```text
transfer::on_client_ready(client_id)
  -> workflow::recover_unfinished_jobs(client_id)
  -> workflow::run_file_gc_loop(client_id)
```

恢复流程：

```text
workflow::recovery::recover_unfinished_jobs(client_id)
  -> store::list_cancelling_jobs()
       -> store::cancel_job_now(...)          # 重启前已请求停止的任务直接收敛为 cancelled
  -> store::list_recoverable_jobs()
       -> spawn_recovery_job(job, client_id)

spawn_recovery_job(job, client_id)
  -> runtime::acquire_transfer_slot()
  -> workflow::resume_one_job(job, client_id)
  -> outcome::send_recovery_outcome(...)
```

单任务恢复会重新 spider 源链接，并把数据库旧子项与当前源消息对齐：

```text
workflow::resume_one_job(job, client_id)
  -> guard::acquire_job_guard(job_id)
  -> control::apply_job_control(job_id)
  -> spider::spider_message(job.source_link, client_id)
  -> store::mark_job_running(job_id)
  -> store::reconcile_items_for_bundle(job_id, bundle, delay_minutes)
       -> 新消息：新增 transfer_item 并增加 file_cache 引用
       -> 消失消息：标记 obsolete 并释放旧 file_cache 引用
       -> 文件变化：迁移 file_key 引用
  -> runner::run_job_inner(job, bundle.messages, client_id)
```

### 进度与查询流程

进度消息不直接驱动任务，只读数据库快照并编辑同一条 Telegram 消息：

```text
progress::update_transfer_progress_message(plan, notify_chat_id, message_id, client_id, done)
  -> store::find_active_job_id_by_source_target(source_link, target_chat_id)
  -> store::get_job_progress_snapshot(job_id)
  -> progress::text::format_transfer_progress_text(...)
  -> progress::keyboard::build_transfer_progress_keyboard(...)
  -> send::edit_markdown_message_with_inline_keyboard(...)
```

`/downloads` 列表查询：

```text
command::downloads::downloads_command(...)
  -> parse_downloads_args(...)
  -> render_downloads_page(...)
       -> store::list_recent_job_snapshots(request_chat_id, query_limit)
       -> DownloadsFilter::matches(snapshot)
       -> render::format_downloads_text(...)
       -> keyboard::build_downloads_keyboard(...)
```

`/downloads` 翻页按钮：

```text
downloads_callback_query(...)
  -> parse_downloads_callback_data(...)
  -> render_downloads_page(...)
  -> send::answer_callback_query(...)
  -> send::edit_markdown_message_with_inline_keyboard(...)
```

### 手动控制流程

`/job` 只修改数据库控制状态；后台工作流会在安全点通过 `apply_job_control` 观察到控制状态。

```text
command::job::job_command(...)
  -> parse_job_args(...)
  -> actions::pause_job(...)
       -> store::pause_job(job_id, request_chat_id)
  -> actions::resume_job(...)
       -> store::wake_job(job_id, request_chat_id)
       -> workflow::is_job_running_in_process(job_id)
       -> spawn_recovery_job(job, client_id)  # 当前进程没有执行器时重新派发
  -> actions::stop_job(...)
       -> store::request_cancel_job(job_id, request_chat_id)
       -> workflow::is_job_running_in_process(job_id)
       -> store::cancel_job_now(...)          # 无执行器时当前命令立即收敛
```

后台执行器里的安全点：

```text
workflow::control::apply_job_control(job_id)
  -> paused      -> TransferOutcome::Paused
  -> cancelling  -> TransferOutcome::Cancelling / Cancelled
  -> running     -> None，继续执行
```

### 发送消息与 Markdown 流程

所有 Markdown 回复最终都会转为 TDLib 原生 `FormattedText`：

```text
send::send_markdown_message(...)
send::send_markdown_message_with_buttons(...)
send::edit_markdown_message_with_inline_keyboard(...)
  -> message::content::parse_markdown_text(...)
       -> tdlib_rs::functions::parse_text_entities(Markdown v1)
```

TDLib 发送消息可能先返回临时 `message_id`，发送成功后再通过 update 返回最终 ID：

```text
sendMessage(...)
  -> send::wait_for_sent_message(temporary_message)
  -> handle_update(Update::MessageSendSucceeded)
       -> send::observe_message_send_succeeded(...)
```

进度消息编辑时如果遇到 `Message not found`，会尝试等待最终 ID 后重试一次：

```text
send::edit_markdown_message_with_inline_keyboard(...)
  -> editMessageText(...)
  -> Message not found
  -> wait_for_sent_message_id(chat_id, temporary_message_id, 30s)
  -> editMessageText(final_message_id)
```

### 文件缓存与删除流程

文件缓存用 `file_key` 去重，`file_key` 优先来自 TDLib remote unique id。

```text
store::ensure_items_for_bundle(...)
  -> file::extract_file_key(message)
  -> store::acquire_file_ref(file_key)

runner::run_job_inner(...)
  -> file::ensure_media_downloaded(...)
  -> store::mark_file_cache_ready(...)

任务完成 / 失败 / 停止
  -> store::finish_job... / store::cancel_job_now(...)
  -> store::release_job_file_refs(...)
  -> active_refs == 0 时设置 delete_after

workflow::run_file_gc_loop(...)
  -> store::list_due_file_cache(...)
  -> store::claim_file_cache_for_delete(...)
  -> tdlib_rs::functions::delete_file(...)
  -> 删除数据库 file_cache 行或记录删除失败
```

## 核心函数索引

| 场景 | 入口函数 | 主要文件 |
| --- | --- | --- |
| TDLib update 主循环 | `tgbot::receive` / `tgbot::handle_update` | `transfer_bot/src/tgbot.rs` |
| `/transfer` 命令 | `transfer_cmd::transfer_command` | `transfer_bot/src/tgbot/transfer/command/transfer_cmd.rs` |
| 后台任务派发 | `spawn_transfer_job` / `spawn_recovery_job` | `transfer_bot/src/tgbot/transfer/spawn.rs` |
| 创建/复用/恢复判断 | `workflow::transfer` / `start::build_transfer_start` | `transfer_bot/src/tgbot/transfer/workflow.rs`, `workflow/start.rs` |
| 核心下载上传状态机 | `runner::run_job_inner` | `transfer_bot/src/tgbot/transfer/workflow/runner.rs` |
| 启动恢复 | `recover_unfinished_jobs` / `resume_one_job` | `transfer_bot/src/tgbot/transfer/workflow/recovery.rs` |
| 文件下载与上传内容 | `ensure_media_downloaded` / `prepare_upload_content` | `transfer_bot/src/tgbot/transfer/file/download.rs`, `file/content.rs` |
| 上传与结果链接 | `upload_prepared` / `build_result_message_link` | `transfer_bot/src/tgbot/transfer/workflow/upload.rs` |
| 进度消息 | `update_transfer_progress_message` / `edit_transfer_progress_for_outcome` | `transfer_bot/src/tgbot/transfer/progress.rs` |
| 任务列表 | `downloads_command` / `format_downloads_text` | `transfer_bot/src/tgbot/transfer/command/downloads.rs`, `downloads/render.rs` |
| 任务控制 | `job_command` / `pause_job` / `resume_job` / `stop_job` | `transfer_bot/src/tgbot/transfer/command/job.rs`, `job/actions.rs` |
| 数据库读写聚合 | `transfer::store` | `transfer_bot/src/tgbot/transfer/store.rs` |
| 消息发送封装 | `send::message` / `send::panel` | `transfer_bot/src/tgbot/send/message.rs`, `send/panel.rs` |

## 数据库说明

项目使用 SeaORM migration 创建表结构，启动时会先读取 `config.json` 的 `storage.database_url`，再连接业务数据库并执行迁移。

核心表：

- `transfer_job`：一次转存请求的主任务，记录源链接、目标 chat、状态和转存结果。
- `transfer_item`：任务中的单个消息或媒体项，记录源消息、文件 key 和子状态。
- `transfer_result_message`：任务上传后产生的结果入口；超过 10 条拆成多个 album 时会保存多个入口。
- `file_cache`：本地文件缓存与引用计数，负责下载去重和延迟删除。

本地 SQLite 数据库属于运行状态，已被忽略，不要提交。默认路径是 `tg/app/transfer.sqlite`，可以通过 `storage.database_url` 改到其它位置。

## 开发检查

常用检查命令：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo fmt -p transfer_bot -- --check
cargo test -p transfer_bot -- --nocapture
cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
```

如果只改文档，可以不跑完整测试；如果改了 `transfer_bot/src`、`migration` 或配置解析逻辑，建议至少跑 `test` 和 `clippy`。

## Git 注意事项

以下文件和目录不应提交：

- `config.json`
- `tg/`
- `*.sqlite`
- `*.sqlite-*`
- `*.log`
- `.idea/`

提交前可以用下面命令确认敏感文件没有进入暂存区：

```powershell
git status --short
git ls-files config.json tg/app/transfer.sqlite transfer_bot/db.sqlite transfer_bot/db.test.sqlite
```

`git ls-files` 没有输出时，说明这些本地状态文件没有被 Git 跟踪。
