# tg_transfer_bot

`tg_transfer_bot` 是一个基于 TDLib 的 Telegram 转存机器人，用于把指定消息、相册或 bot 可见媒体转存到目标聊天，并把任务状态、文件缓存和恢复信息持久化到本地 SQLite。

它的重点不是“能发一次消息”，而是“能稳定长期跑”：

- 支持后台任务恢复，进程重启后继续未完成转存。
- 支持文件下载去重和引用计数，避免重复下载同一媒体。
- 支持进度卡片、任务列表、暂停/恢复/停止和历史查询。
- 支持 bot 优先读取源，失败时回退到 user client。
- 支持菜单输入草稿持久化，未完成交互可在重启后继续。
- 支持管理员和普通用户权限隔离，普通用户按积分使用。

## 适用场景

- 把频道、群组或私聊中的媒体转存到归档群。
- 用 bot 统一接收 `/transfer` 指令，但允许 user client 补足 bot 无法读取的源。
- 在需要恢复、查询和任务控制的环境里长期运行，而不是一次性脚本。

## 主要能力

- `/transfer` 转存单条消息、相册或回复的 bot 可见媒体。
- `/menu` 卡片式入口，覆盖转存、查询、任务控制、配置、健康和缓存。
- `/downloads` 查看任务列表、状态筛选、分页和真实下载进度。
- `/lookup` 按源链接查询已成功转存结果。
- `/job` 手动暂停、恢复、停止任务。
- `/balance` 查看当前用户积分余额和积分流水。
- `/points` 由管理员查看、查询流水、增加或扣减普通用户积分。
- `/config` 动态调整已开放的转存运行时参数并写回配置文件。
- `/health` 查看运行健康、并发和缓存摘要。
- `/cache` 查看文件缓存概览和分页明细。

## 项目结构

```text
.
├── config.example.json     # 本地配置模板
├── session.md              # 跨会话恢复文档
├── Cargo.toml              # workspace 配置
├── tdlib_rs/               # TDLib Rust 绑定
└── transfer_bot/           # 机器人主程序
```

日常业务开发主要集中在 `transfer_bot/src/tgbot/transfer/`。`tdlib_rs` 主要是绑定层，通常不需要手工修改。

## 快速开始

### 1. 准备 TDLib

项目通过 `tdjson` 连接 TDLib。构建和运行前需要设置 `LOCAL_TDLIB_PATH`。

PowerShell 示例：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
```

该目录通常至少包含：

- `include/`
- `lib/`
- `bin/tdjson.dll`

### 2. 创建本地配置

以 [config.example.json](config.example.json) 为模板创建 `config.json`。真实配置不要提交到仓库。

最小可运行配置通常需要先填好这些值：

- `tdlib_defaults.api_id`
- `tdlib_defaults.api_hash`
- `clients.user.login_info`
- `clients.user.tdlib.database_encryption_key`
- `clients.bot.token`
- `clients.bot.tdlib.database_encryption_key`
- `access_control.admin_user_ids`
- `access_control.allowed_request_chat_ids`
- `access_control.allowed_target_chat_ids`
- `targets.default_chat_id`

### 3. 启动

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo run -p transfer_bot -- -c config.json
```

首次运行时：

- `user` client 可能需要二维码、验证码或二次密码登录。
- `bot` client 会使用 `token` 登录。
- 程序会自动创建业务 SQLite 库和运行目录。

## 配置说明

### 配置分层

- `tdlib_defaults`：user/bot 共用的 TDLib 公共参数。
- `storage`：业务数据库位置，保存任务、缓存、恢复和菜单草稿。
- `clients.user` / `clients.bot`：两个 Telegram client 的本地目录和登录方式。
- `workflow`：上传使用哪个 client。
- `access_control`：哪些人、哪些 chat 可以发命令和作为目标。
- `billing`：普通用户积分计费规则。
- `targets`：默认目标 chat、按请求 chat 的映射和目标别名。
- `transfer_config`：并发、GC、进度刷新、分页、菜单超时等运行参数。

### 关键字段

| 字段 | 说明 |
| --- | --- |
| `config_version` | 当前配置版本，现为 `2` |
| `storage.database_url` | 业务 SQLite 连接串，默认 `sqlite://tg/app/transfer.sqlite?mode=rwc` |
| `clients.user.login_info` | user 登录方式，支持 `OCR`、`PHONE` |
| `clients.bot.token` | BotFather 生成的 bot token |
| `workflow.upload_client` | 上传使用 `bot` 或 `user` |
| `access_control.admin_user_ids` | 允许管理机器人的用户 ID |
| `access_control.allowed_user_ids` | 允许作为普通用户私聊 bot 的用户 ID |
| `access_control.allow_all_private_users` | 是否允许任意私聊用户作为普通用户使用 |
| `access_control.banned_user_ids` | 禁止使用 bot 的用户 ID |
| `access_control.allowed_request_chat_ids` | 允许管理员发命令的非私聊 chat ID |
| `access_control.allowed_target_chat_ids` | 允许转存到的目标 chat ID |
| `billing.enabled` | 是否启用普通用户积分计费 |
| `billing.base_cost_points` | 每次转存基础成本 |
| `billing.item_cost_points` | 每条源消息成本 |
| `billing.initial_user_points` | 普通用户首次创建账号时发放的初始积分 |
| `targets.default_chat_id` | 未显式指定目标时的默认目标 |
| `targets.by_request_chat_id` | 按请求 chat 映射默认目标 |
| `targets.aliases` | 目标别名，例如 `archive` |
| `transfer_config.job_concurrency` | 后台转存任务并发数 |
| `transfer_config.file_delete_delay_minutes` | 文件引用归零后的延迟删除时间 |
| `transfer_config.file_gc_interval_seconds` | 文件 GC 扫描间隔 |
| `transfer_config.progress_edit_interval_seconds` | 进度卡片最短编辑间隔 |
| `transfer_config.downloads_default_page_size` | `/downloads` 默认页大小 |
| `transfer_config.menu_input_timeout_seconds` | 菜单输入超时 |

### 推荐 workflow

```json
{
  "workflow": {
    "upload_client": "bot"
  }
}
```

默认推荐 `bot` 上传。当前实现的源读取策略是：

- 链接源优先走 `bot`。
- `bot` 无法读取或准备文件时自动回退 `user`。
- 最终上传端由 `workflow.upload_client` 决定。

`/config` 只开放 `transfer_config` 中的运行时参数，例如并发、GC 间隔、进度刷新、分页大小和菜单超时。TDLib 登录、API ID、API Hash、bot token、数据库目录等启动级配置仍需手工修改配置文件后重启生效。

重复转存的业务语义固定是 `source_link + target_chat_id`，不区分上传端。

### 访问控制与目标解析

命令是否会被处理，取决于发送者身份：

- admin：`sender_user_id` 必须在 `access_control.admin_user_ids` 中；可在私聊或 `access_control.allowed_request_chat_ids` 里的 chat 操作。
- 普通用户：只能私聊 bot；必须在 `access_control.allowed_user_ids` 中，或开启 `allow_all_private_users`。
- banned：只要命中 `access_control.banned_user_ids` 就直接拒绝。

权限边界：

- admin 可查看和控制全局任务，不扣积分，可使用 `user` fallback。
- 普通用户只能查看和控制自己的任务，重复转存和 lookup 也只复用自己的结果。
- 普通用户通过链接转存时不允许借 `user` 账号 fallback 读取私有源，避免越权读取。
- 普通用户在 spider 成功后、创建 job 前扣积分；无效链接不会扣费。
- 任务全部失败或用户停止时会全额退回本次扣费；部分成功会按失败条目占比退回本次扣费。

普通用户常见失败会显示可执行提示：

- 余额不足：提示查看 `/balance` 和 `/balance history`，并联系管理员加分。
- 目标不可用：提示目标不在 `allowed_target_chat_ids`，需要换目标或让管理员更新白名单。
- 源不可访问：提示 bot 无法读取源消息；普通用户不能借 `user` fallback，应转发源消息给 bot 或让 bot 加入源聊天。

`/transfer` 未显式传目标时，目标 chat 的解析顺序是：

1. 命令参数里显式给出的数字 chat ID 或 `targets.aliases` 别名。
2. `targets.by_request_chat_id[request_chat_id]`。
3. `targets.default_chat_id`。

如果配置了 `access_control.allowed_target_chat_ids`，那么无论目标来自显式参数、别名还是默认映射，最终都必须命中白名单。

### 本地状态文件

这些文件或目录属于本地运行状态，不应提交：

- `config.json`
- `tg/`
- `*.sqlite`
- `*.sqlite-*`
- `*.log`

## 常用命令

| 命令 | 短命令 | 作用 |
| --- | --- | --- |
| `/help [command]` | `/h [command]` | 查看命令目录或单个命令帮助 |
| `/menu` | `/m` | 打开交互菜单 |
| `/transfer [link] [target]` | `/t [link] [target]` | 创建转存任务或进入向导 |
| `/lookup <link> [target]` | `/lk <link> [target]` | 查询历史成功转存结果 |
| `/downloads [filter] [limit] [page]` | `/d [filter] [limit] [page]` | 查看任务列表和下载进度 |
| `/job <pause|resume|stop> <job_id>` | `/j <p|r|s> <job_id>` | 控制任务 |
| `/balance` | `/bal` | 查看当前用户积分余额 |
| `/balance history [limit] [page]` | `/bal h [limit] [page]` | 查看当前用户积分流水 |
| `/points <show|history|add|sub> <user_id> [amount|limit] [reason|page]` | `/pts <s|h|a|sub> ...` | 管理员查看、查询流水或调整用户积分 |
| `/config [show|set <key> <value>]` | `/cfg [show|set <key> <value>]` | 查看或调整已开放的运行时参数 |
| `/health` | `/hl` | 查看健康状态、并发和缓存摘要 |
| `/cache [summary|page] [limit] [page]` | `/fc [summary|page] [limit] [page]` | 查看文件缓存 |

### 常见使用方式

直接转存链接：

```text
/transfer https://t.me/c/123/456
/transfer https://t.me/c/123/456 archive
/transfer https://t.me/c/123/456 -1001234567890
```

回复 bot 可见媒体消息：

```text
/transfer archive
```

进入菜单向导：

```text
/menu
-> 首页显示运行摘要
-> 如果存在未完成输入，首页顶部显示“继续输入 / 取消输入”
-> 第一行：开始转存 / 快速转存 / 快速查询
-> 第二行：运行任务 / 失败任务 / 已暂停
-> 其他入口：下载列表 / 任务控制 / 转存页 / 查询页 / 运行配置 / 帮助 / 运行健康 / 文件缓存 / 用户流水
```

菜单中的“选择群组”使用 Telegram 原生 `keyboardButtonTypeRequestChat`。这个入口主要适用于 bot 私聊，最终选中的目标仍会经过 `allowed_target_chat_ids` 校验。
admin 首页的“用户流水”会先让你回复 Telegram user_id，再打开该用户的积分流水卡片。

当前转存向导是 3 步：

```text
1/3 等待源链接
2/3 选择目标（上次目标 / 默认目标 / 别名 / 选择群组 / 手动输入）
3/3 确认执行
```

`快速转存` 和 `快速查询` 会优先使用默认目标；如果当前请求 chat 或全局没有默认目标，会自动退回普通“选择目标 -> 确认”流程。

查看任务：

```text
/downloads
/downloads run
/downloads fail 5
```

`/downloads` 支持的筛选值为：

```text
all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel
```

控制任务：

```text
/job p 123
/job r 123
/job s 123
```

积分：

```text
/balance
/balance history
/balance history 10 2
/points show 123456789
/points history 123456789 10 1
/points add 123456789 10 admin_adjust
/points sub 123456789 10 admin_adjust
```

积分流水卡片支持 `首页 / 上页 / 下页 / 末页 / 刷新 / 菜单` 按钮；当前页按钮保留可复制命令，方便手动排查。

## 运行机制

### 用户视角

1. 用户发送 `/transfer`、菜单指令，或回复一条 bot 可见媒体。
2. 机器人立即回复一张进度卡片。
3. 后台解析源消息，识别单条或相册。
4. 普通用户按 `billing` 配置扣积分；admin 不扣积分。
5. 任务和子项先写入数据库，再开始下载和上传。
6. 进度卡片通过编辑同一条消息持续更新。
7. 完成、失败或停止后，结果和状态会持久化；全部失败或停止会幂等全额退款，部分成功会按失败条目占比退款。
8. 文件引用归零后进入延迟删除队列。
9. 程序重启后自动恢复未完成任务。

### 内部设计要点

- 业务数据库和 TDLib 自身数据库分离。
- `transfer_job` 表示一次转存请求，`transfer_item` 表示任务中的单个源消息。
- `file_cache` 负责文件去重、引用计数和延迟删除。
- `transfer_result_message` 保存一个任务可能对应的多个结果入口。
- `user_account` 保存用户角色和积分余额。
- `point_ledger` 保存积分增加、扣减和退款流水。
- 创建阶段按 `source_link + target_chat_id` 做业务去重。
- 同一条命令按 `request_chat_id + request_message_id` 做幂等保护。

## 日志与排查

默认日志会输出到控制台和仓库根目录的 `tg_transfer.log`。

常规排查：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
$env:RUST_LOG = "transfer_bot=debug,info,sea_orm=warn,sqlx=warn,tokio=warn"
cargo run -p transfer_bot -- -c config.json
```

需要追踪更细的 TDLib update 或下载进度时：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
$env:RUST_LOG = "transfer_bot=trace,info,sea_orm=warn,sqlx=warn,tokio=warn"
cargo run -p transfer_bot -- -c config.json
```

常见日志关键词：

| 关键词 | 含义 |
| --- | --- |
| `tdlib authorization ready` | 对应 client 登录成功 |
| `starting transfer background services` | 恢复和 GC 后台服务已启动 |
| `ignored historical message` | 启动前历史消息被过滤 |
| `ignored non-admin message` | 不在访问控制范围内的消息被忽略 |
| `bot command received` | bot 收到并准备处理命令 |
| `transfer points charged` | 普通用户转存已扣积分 |
| `transfer points refunded` | 失败或停止任务已退回积分 |
| `transfer background task queued` | 新转存任务已入队 |
| `transfer job execution started` | 开始执行下载/上传 |
| `tdlib receive loop exited with error` | 主接收循环异常退出 |

## 加密配置

如果需要把配置文件加密保存，可以先生成密文：

```powershell
cargo run -p transfer_bot -- -c config.json encrypt <password>
```

再使用密文启动：

```powershell
cargo run -p transfer_bot -- -c config.json.enc decrypt <password>
```

## 数据库说明

业务数据库默认路径是 `tg/app/transfer.sqlite`，由 `storage.database_url` 控制。程序启动时会直接确保当前代码所需的表结构存在，当前开发阶段不维护单独 migration 历史。

核心表：

- `transfer_job`：主任务记录。
- `transfer_item`：任务子项。
- `transfer_result_message`：结果入口记录。
- `file_cache`：文件缓存、引用计数、删除状态。
- `menu_input_draft`：菜单输入草稿和超时信息。
- `user_account`：用户角色、余额和累计增加/消费。
- `point_ledger`：积分流水和扣费幂等键。

TDLib 的 `tg/user/db`、`tg/bot/db` 是 Telegram client 自身状态库，不等同于业务库。

开发期如果需要重建业务数据库，可以直接删除 `tg/app/transfer.sqlite`。下次启动时程序会自动按当前代码重新建表。不要删除 `tg/user/db`、`tg/bot/db` 这些 TDLib 状态目录。

## 开发入口

如果要读主流程，建议从这些文件开始：

- `transfer_bot/src/lib.rs`：新的库入口，负责应用装配、配置读取、数据库初始化和启动 TDLib 接收循环。
- `transfer_bot/src/main.rs`：当前仅保留 Tokio runtime 启动器。
- `transfer_bot/src/app_context.rs`：应用级共享上下文，集中持有转存运行配置、下载进度、singleflight、执行 guard 和发送能力。
- `transfer_bot/src/tgbot.rs`：TDLib update 接收和总分发入口。
- `transfer_bot/src/tgbot/login.rs`：TDLib 授权状态机。
- `transfer_bot/src/tgbot/error.rs`：命令错误提示、权限拒绝和自动转存引导卡片。
- `transfer_bot/src/tgbot/send/error.rs`：统一交互错误卡片。
- `transfer_bot/src/tgbot/transfer/command/transfer_cmd.rs`：`/transfer` 入口和参数解析。
- `transfer_bot/src/tgbot/transfer/command/points.rs`：`/balance` 与 `/points` 命令入口。
- `transfer_bot/src/tgbot/transfer/command/points/render.rs`：积分余额、积分流水卡片和分页按钮渲染。
- `transfer_bot/src/tgbot/transfer/command/points/callback.rs`：积分流水翻页和刷新 callback。
- `transfer_bot/src/tgbot/transfer/spawn.rs`：后台任务派发和进度卡片生命周期。
- `transfer_bot/src/tgbot/transfer/workflow/start.rs`：复用、恢复、新建任务的决策层。
- `transfer_bot/src/tgbot/transfer/workflow/runner.rs`：下载、准备、上传和终态写入的核心执行器。
- `transfer_bot/src/tgbot/transfer/workflow/recovery.rs`：启动恢复逻辑。
- `transfer_bot/src/tgbot/transfer/store/`：数据库读写聚合层。
- `transfer_bot/src/db.rs`：业务表结构定义和 schema 初始化。

### 主调用链

```text
main
  -> transfer_bot::run()
     -> load config
     -> init business db
     -> build AppContext
     -> create tdlib clients
     -> tgbot::receive(app_context, config)
        -> handle_update(app_context, ...)
           -> login / commands / callback / file progress
```

### 当前重构状态

当前正在做一轮“架构优先”的重构，目标是把 `transfer_bot` 从强全局状态的二进制程序整理成有明确库入口和应用上下文的服务。已经完成的部分：

- `main.rs` 已缩成纯启动器，主装配逻辑迁移到 `transfer_bot/src/lib.rs`。
- 新增 `transfer_bot/src/app_context.rs`，把转存运行配置、singleflight、下载进度、执行 guard 和发送能力收进统一上下文。
- `run -> receive -> handle_update -> handle_authorization -> on_clients_ready` 这条主链已经开始显式传 `AppContext`。
- 后台链 `on_clients_ready -> spawn -> workflow/recovery/gc/progress` 已经开始逐步改成直接使用 `AppContext` 中的运行时状态。
- `workflow::transfer`、`resume_one_job` 和 `runner` 入口现在也已显式接收 `AppContext`，为后续继续下推到执行细节做准备。
- `health` / `observability` 这条只读查询链已经开始直接从 `AppContext` 读取运行时配置和活跃任务计数。
- 交互层也开始收口状态语义：任务状态到“列表入口/按钮文案/可用控制”的映射正逐步统一，避免菜单、下载列表、任务详情和进度卡片各自编码一套规则。
- `/balance` 与 `/points` 已按命令入口、流水渲染、callback 处理拆分，避免积分命令继续堆成单个大文件。
- 恢复链路新增专项测试，覆盖 source-target 复用、重复活跃任务、同请求幂等取消和恢复扫描边界。

这轮重构的业务目标不是改功能，而是先降低后续优化和测试演进的耦合成本。

`/transfer` 的主链路：

```text
transfer_command
  -> dispatch_transfer_plan
  -> spawn_transfer_job
  -> workflow::transfer
     -> build_transfer_start
     -> create/reuse/resume job
     -> runner::run_job_inner
```

## 开发检查

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo fmt -p transfer_bot -- --check
cargo test -p transfer_bot -- --nocapture
cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
```

最近一次完整验证结果：

- `cargo check -p transfer_bot` 通过
- `cargo test -p transfer_bot` 通过，`263 passed`
- `cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 通过

注意：运行测试会重新生成 `transfer_bot/db.test.sqlite`。如果希望工作区里不保留业务数据库，测试结束后需要手动删除它。

注意：运行测试会重新生成 `transfer_bot/db.test.sqlite`。如果你希望工作区里不保留业务数据库，测试结束后需要手动删掉它。

如果只改文档，可以不跑完整测试；如果改了 `transfer_bot/src`、配置解析或任务状态流转，建议至少跑 `test` 和 `clippy`。

## 提交前检查

```powershell
git status --short
git ls-files config.json tg/app/transfer.sqlite transfer_bot/db.sqlite transfer_bot/db.test.sqlite
```

如果 `git ls-files` 没有输出，说明这些本地状态文件没有被 Git 跟踪。
