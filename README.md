# tg_transfer_bot

`tg_transfer_bot` 是一个基于 TDLib 的 Telegram 转存机器人，用于把指定消息、相册或 bot 可见媒体转存到目标聊天，并把任务状态、文件缓存和恢复信息持久化到业务数据库。

它的重点不是“能发一次消息”，而是“能稳定长期跑”：

- 支持后台任务恢复，进程重启后继续未完成转存。
- 支持文件下载去重和引用计数，避免重复下载同一媒体。
- 支持进度卡片、任务列表、暂停/恢复/停止和历史查询；按钮停止会先进入确认页。
- 支持 bot 优先读取源，失败时回退到 user client。
- 支持菜单输入草稿持久化，未完成交互可在重启后继续。
- 采用授权私聊模式，支持 `owner_user_id`、`admin_user_ids` 和数据库动态授权名单。

## 适用场景

- 把频道、群组或私聊中的媒体转存到归档群。
- 用 bot 统一接收 `/transfer` 指令，但允许 user client 补足 bot 无法读取的源。
- 在需要恢复、查询和任务控制的环境里长期运行，而不是一次性脚本。

## 主要能力

- `/transfer` 转存单条消息、相册或回复的 bot 可见媒体。
- `/menu` 卡片式入口，覆盖转存、查询、任务控制、配置、健康和缓存。
- `/downloads` 查看任务列表、状态筛选、分页和真实下载进度。
- `/lookup` 按源链接查询已成功转存结果，并返回 URL 或 Telegram 消息引用入口。
- `/job` 手动暂停、恢复、停止任务；按钮入口的停止动作会二次确认。
- `/auth` 打开 owner 专用授权面板，可查看管理员名称、选择 Telegram 用户、输入 ID 或删除动态管理员；修改后立即生效并持久化。
- `/targets` 管理默认目标和目标别名。
- `/config` 动态调整已开放的转存运行时参数并写入数据库。
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
- `clients.user.tdlib.database_encryption_key`
- `clients.bot.token`
- `clients.bot.tdlib.database_encryption_key`
- `owner_user_id`
- `admin_user_ids`（可选，其他同权管理员的 Telegram 用户 ID）

### 3. 启动

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo run -p transfer_bot -- -c config.json
```

首次运行时：

- `bot` client 会使用 `token` 登录。
- Bot 登录后即可使用；需要 user 执行器时，仅 owner 在“管理 -> 执行器”中点击登录并扫描二维码。
- 程序会自动执行数据库 migration；SQLite 文件库会自动创建运行目录。

首次启动后，用 `owner_user_id` 或 `admin_user_ids` 中的账号私聊 bot 并打开 `/menu`，按下面顺序完成运行态检查：

1. `目标配置`：按需配置默认目标或目标别名；不配置时默认目标就是当前私聊。
2. `运行配置`：按需要调整并发、删除延迟、分页和菜单超时。
3. `运行健康`、`文件缓存`：确认后台任务和文件状态正常。

需要增加使用者时，由 owner 发送 `/auth`，点击“添加管理员”，再选择 Telegram 用户或输入用户 ID。动态授权和名称快照会写入业务数据库，重启后自动恢复；`/auth add <user_id>` 仍作为命令行兜底。

`targets` 是可选运行态配置；不设置默认目标时，快速转存会回落到当前私聊。

## GitHub Actions

仓库内提供三个手动 workflow：

- `release-packages.yml`：面向正式发布，手动构建全部目标系统。
- `test-single-target.yml`：面向单系统验证，手动只构建一个目标系统。
- `publish-tag-release.yml`：选择已有 tag 或分支，构建全部目标；tag 可继续发布为 GitHub Release。

### release-packages.yml

在 GitHub Actions 页面手动运行后可选：

- `package_mode`
  - `package`：编译、打包并上传 artifact。
  - `build_only`：只验证能否编译，不上传打包文件。
- `td_ref`
  - TDLib 的 `branch`、`tag` 或 `commit`，默认 `master`。
- `run_checks`
  - 是否先执行 `cargo fmt`、`cargo test`、`cargo clippy`。

### publish-tag-release.yml

所有参数均为下拉框：

- `source_ref`：选择构建来源，当前可选 `dev`、`master` 和 `v0.0.1`。
- `release_mode`
  - `build_only`：只构建并上传 workflow artifact，适合使用分支测试。
  - `publish_release`：构建后发布 GitHub Release，只允许搭配 tag 使用。
- `td_ref`：选择 TDLib 版本来源，当前为 `master`。
- `run_checks`：选择是否先执行格式化、测试和 Clippy 检查。

默认使用 `dev + build_only`，避免测试时误创建 Release。GitHub Actions 的 `workflow_dispatch.choice` 不支持动态读取仓库 tag 或分支；创建新 tag 或分支后，需要同时将名称加入 `publish-tag-release.yml` 的 `source_ref.options`。

当前正式发布目标：

- `linux-x86_64-alpine3.23`
- `linux-x86_64-debian13`
- `linux-x86_64-ubuntu24.04`
- `windows-x86_64-msvc`

### test-single-target.yml

在 GitHub Actions 页面手动运行后可选：

- `target`
  - `alpine-3.23`
  - `debian-13`
  - `ubuntu-24.04`
  - `windows-2022`
- `package_mode`
  - `package`：编译、打包并上传单个测试 artifact。
  - `build_only`：只验证单个目标能否编译。
- `td_ref`
- `run_checks`

### 产物说明

- Linux 产物为 `.tar.gz`，包含 `bin/transfer_bot`、`libtdjson.so` 和运行时依赖。
- Windows 产物为 `.zip`，包含 `bin/transfer_bot.exe`、`tdjson.dll` 和相关 DLL。
- 所有正式产物默认附带 `.sha256` 校验文件；测试 workflow 默认只上传主压缩包。

## 配置说明

### 配置分层

- `tdlib_defaults`：user/bot 共用的 TDLib 公共参数。
- `storage`：业务数据库位置，保存任务、缓存、恢复、菜单草稿和动态授权名单。
- `clients.user` / `clients.bot`：两个 Telegram client 的本地目录；user 由 owner 在 Bot 内按需二维码登录。
- `owner_user_id`：必填的所有者 Telegram 用户 ID。
- `admin_user_ids`：可选的同权管理员 Telegram 用户 ID 白名单。
- 运行参数和目标配置以数据库为准，通过 `/config`、`/targets` 或菜单管理。

### 关键字段

| 字段 | 说明 |
| --- | --- |
| `config_version` | 当前配置版本，现为 `2` |
| `owner_user_id` | 所有者用户 ID；必须大于 `0`，始终拥有完整权限 |
| `admin_user_ids` | 其他同权管理员用户 ID 数组；ID 必须大于 `0`，可为空 |
| `storage.database_url` | 业务数据库连接串，支持 `sqlite://...`、`postgres://...`、`postgresql://...` |
| `clients.user.login_info` | 兼容旧配置；仅接受 `OCR`，新配置可省略 |
| `clients.bot.token` | BotFather 生成的 bot token |
| `workflow.upload_client` | 兼容旧配置；仅接受 `bot`，新配置可省略 |

以下字段不再建议写入 `config.json`，运行时以数据库为准：

| 数据库配置 | 管理方式 |
| --- | --- |
| 默认目标、目标别名 | `/targets show`、`/targets ...` 或菜单“管理 -> 目标配置” |
| 并发、文件清理、分页、菜单输入超时 | `/config show`、`/config ...` 或菜单“管理 -> 运行配置” |
| 动态授权用户 | 仅 owner 使用 `/auth` 交互面板；也可使用 `/auth list`、`/auth add <user_id>`、`/auth del <user_id>` |

默认由 `bot` 读取、下载和上传。当前实现的源读取策略是：

- 链接源优先走 `bot`。
- `bot` 无法读取或准备文件且执行器已登录时，自动回退 `user`。
- 执行器未登录时，任务会明确提示需要在“管理 -> 执行器”完成登录。

`/config` 只开放运行时参数，例如并发、GC 间隔、进度刷新、分页大小和菜单超时。命令修改会直接写入业务数据库并立即生效，不再回写 `config.json`。TDLib 登录、API ID、API Hash、bot token、数据库目录等启动级配置仍需手工修改配置文件后重启生效。

重复转存的业务语义固定是 `source_link + target_chat_id`，不区分上传端。

### 管理员边界与目标解析

只有同时满足以下条件的交互才会被处理：

- `chat_id == sender_user_id`
- `sender_user_id == owner_user_id`，或 `admin_user_ids` / 数据库动态授权名单包含该用户 ID

也就是说，项目只接受已授权用户的 bot 私聊；群聊、频道不执行命令，未授权私聊会收到“无权限，请联系管理员”提示。

交互边界：

- 本项目不支持在群聊里发命令或点击交互按钮。
- 目标群只作为转存目的地出现，需要在私聊菜单里选择或通过命令参数指定。
- 目标选择支持 Telegram 原生群组/频道选择器，也可使用当前私聊、默认目标、已有别名或手动输入 chat ID。

所有已授权用户均可使用转存、查询和任务管理功能；只有 `owner_user_id` 可以执行 `/auth`，避免其他用户继续扩散权限。链接源优先由 bot 读取；bot 无法读取或准备文件时，会尝试 TDLib `user` client fallback。

`/transfer` 未显式传目标时，目标 chat 的解析顺序是：

1. 命令参数里显式给出的数字 chat ID 或 `targets.aliases` 别名。
2. `targets.default_chat_id`。
3. 未配置默认目标时回落到当前管理员私聊。

### 本地状态文件

这些文件或目录属于本地运行状态，不应提交：

- `config.json`
- `tg/`
- `*.sqlite`
- `*.sqlite-*`
- `*.log`

## 常用命令

所有命令仅对静态或动态授权用户开放；其中 `/auth` 仅 owner 可执行：

| 命令 | 作用 |
| --- | --- |
| `/help [command]` | 查看命令目录或单个命令帮助 |
| `/menu` | 打开交互菜单 |
| `/transfer [link] [target]` | 创建转存任务或进入向导；不填 `target` 时使用预配置目标 |
| `/lookup <link> [target]` | 查询历史成功转存结果 |
| `/downloads [filter] [limit] [page]` | 查看全部任务列表和下载进度 |
| `/job <pause|resume|stop|status> <job_id>` | 控制任务 |
| `/auth [list|add <user_id>|del <user_id>]` | 打开管理员列表和交互式添加/删除面板；参数命令保留为兜底（仅 owner） |
| `/config [show|reset|set <key> <value>]` | 查看、重置或调整已开放的运行时参数 |
| `/targets [show|reset|set-default|set-alias|del-alias]` | 管理默认目标和目标别名 |
| `/health` | 查看健康状态、并发和缓存摘要 |
| `/cache [summary|page] [limit] [page]` | 查看文件缓存 |

### 常见使用方式

直接转存链接：

```text
/transfer https://t.me/c/123/456
/transfer https://t.me/c/123/456 archive
/transfer https://t.me/c/123/456 -1001234567890
```

私聊 bot 直接发送一条单独的 `t.me` 链接文本时：

```text
不需要先补 /transfer
-> 直接进入“选择目标 -> 确认执行”流程
```

回复 bot 可见媒体消息：

```text
/transfer archive
```

私聊 bot 直接发送一条 bot 可见媒体时：

```text
不需要先补 /transfer
-> 直接进入“选择目标 -> 确认执行”流程
```

私聊 bot 转发一条能还原原始 chat/message_id 的消息时：

```text
也会直接进入“选择目标 -> 确认执行”流程
```

如果转发消息无法还原稳定原始 message_id：

```text
不会伪造当前转发壳消息作为源
-> 会提示你改用原始消息链接
-> 或回复一条 bot 可见媒体后再试
```

进入菜单向导：

```text
/menu
-> 首页显示运行摘要
-> 如果存在未完成输入，首页顶部显示“继续输入 / 取消输入”
-> 首页：开始转存 / 快速转存
-> 导航：任务 / 管理 / 帮助
-> 管理：运行配置 / 目标配置 / 授权管理（仅 owner）/ 运行健康 / 文件缓存
```

当前转存向导是 3 步：

```text
1/3 等待源链接
2/3 选择目标（选择聊天 / 上次目标 / 当前私聊或默认目标 / 别名 / 手动输入）
3/3 确认执行
```

转存完成后的跳转入口按 Telegram 能力区分：频道和超级群提供 `https://t.me/...` 按钮；私聊和 basic group 没有独立消息 URL，结果通知会原生回复目标消息，点击消息引用即可跳转，并保留 `chat_id/message_id` 定位作为降级信息。

`快速转存` 和 `快速查询` 会优先使用默认目标；如果没有显式默认目标，会直接回落到当前私聊。

运行态管理页也支持按钮进入输入流：

```text
/menu
-> 管理
-> 目标配置 / 运行配置 / 运行健康 / 文件缓存
```

其中两个配置页支持输入式流程：

```text
目标配置
- 刷新 / 重置默认 / 恢复私聊默认：直接 callback 执行
- 设默认：使用 Telegram 原生按钮选择群组或频道，也可直接输入 target_chat_id
- 设别名：先回复 alias，再选择目标聊天或输入 target_chat_id
- 现有别名：先点编号进入详情，再改目标 / 设默认 / 删除
- 别名搜索：先输入关键字，再在搜索结果里点编号进入详情

授权管理（仅 owner）
- 添加管理员：点击按钮后选择 Telegram 用户，也可手动输入 user_id
- 快捷授权：回复对方消息并发送 /auth
- 管理员列表：显示名称、username 和 user_id，可点击按钮撤销动态授权

运行配置
- 可先点字段详情：并发 / 删除 / GC / 进度 / 分页 / 超时
- 详情页里可微调，也可进入 ForceReply 回复一个值
- 主页仍保留快捷微调和输入入口
```

这些输入都复用同一套菜单草稿状态：

```text
- 发送 /cancel 可取消
- 原生选择器也可直接点“取消”
- 超时后会提示输入已过期
- 重新打开 /menu 可继续输入
- 中途发送其他命令时，命令优先
```

Telegram 的 `/` 命令菜单仍会注册完整命令，方便需要时直接调用；普通状态卡片默认不展开命令文本，可点击“查看命令”后再查看。

两个运行态配置页统一按钮层级为：

```text
1. 主操作
2. 详情 / 输入
3. 帮助 / 菜单
4. 命令模板 / 兜底复制
```

两个配置页内部使用规格表驱动：

```text
/config  -> ConfigFieldSpec
/targets -> TargetsInputSpec
```

也就是说，以下信息不再散落在多处手写维护：

- callback 参数
- 按钮文案
- ForceReply 标题、说明、placeholder
- 示例命令
- help 交互入口
- 管理输入最终要复用的原始命令

菜单输入分发按规格反查所属命令模块，再统一分发到原命令入口。

`/help` 体系只负责导航和说明。详情页优先使用真实 callback / 输入流；命令示例仍在正文中展示。

首启初始化与清库后重配建议按这个顺序执行：

```text
1. 启动程序，等待 bot 和 user 都登录完成。
2. 使用 `owner_user_id` 或 `admin_user_ids` 中的账号私聊 bot。
3. 进入 目标配置：
   - 默认目标和别名都是可选项
   - 不配置默认目标时，转存目标默认回落到当前私聊
4. 进入 运行配置：
   - 检查并发、文件删除延迟、GC 间隔、进度刷新、分页大小、菜单超时
5. 配完后可发送：
   - /targets show
   - /config show
   逐项确认数据库运行态是否符合预期。
```

如果你删除了业务数据库但保留了 `tg/user`、`tg/bot` 的 TDLib 数据目录，重新启动后的恢复方式也是同一套：

```text
1. 保留 config.json 中的 `owner_user_id` 和 `admin_user_ids`。
2. 重新启动程序。
3. 按需重新配置 targets / config。
4. 旧任务、缓存和运行态配置会丢失；TDLib 登录态和本地媒体目录仍保留。
5. 完成重配后再开始新的转存任务。
```

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
/job status 123
/job pause 123
/job resume 123
/job stop 123
```

## 运行机制

### 用户视角

1. 所有者发送 `/transfer`、菜单指令，或回复一条 bot 可见媒体。
2. 机器人立即回复一张进度卡片。
3. 后台解析源消息，识别单条或相册。
4. 任务和子项先写入数据库，再开始下载和上传。
5. 进度卡片通过编辑同一条消息持续更新。
6. 完成、失败或停止后，结果和状态会持久化。
7. 文件引用归零后进入延迟删除队列。
8. 程序重启后自动恢复未完成任务。

任务详情、下载列表、最近任务和 lookup 命中运行任务时，`停止` 按钮只会打开确认卡片；确认卡片里的 `确认停止` 才会执行真实停止。旧消息里已经存在的停止 callback 仍然兼容，不会因为协议更新失效。

### 内部设计要点

- 业务数据库和 TDLib 自身数据库分离。
- `transfer_job` 表示一次转存请求，`transfer_item` 表示任务中的单个源消息。
- `file_cache` 负责文件去重、引用计数和延迟删除。
- `transfer_result_message` 保存一个任务可能对应的多个结果入口。
- 创建阶段按 `source_link + target_chat_id` 做业务去重。
- 同一条命令按 `request_chat_id + request_message_id` 做幂等保护。

### 开发阅读入口

如果你要继续读代码，建议按这个顺序看：

```text
1. transfer_bot/src/tgbot/transfer/command.rs
   看所有命令入口和 callback 分发总线。

2. transfer_bot/src/tgbot/transfer/command/menu.rs
   看 /menu 首页、hub 页面和 callback 总路由。

3. transfer_bot/src/tgbot/transfer/command/menu/input.rs
   看菜单输入主流程：
   - 草稿读取
   - ForceReply 继续输入
   - 管理输入分发

4. transfer_bot/src/tgbot/transfer/command/menu/input/state.rs
   看菜单草稿状态、AdminInputAction、持久化编码和恢复逻辑。

5. transfer_bot/src/tgbot/transfer/command/menu/input/admin.rs
   看管理输入如何按规格转成原始命令：
   - parse_admin_input_payload
   - admin_command_kind

6. transfer_bot/src/tgbot/transfer/command/config_cmd.rs
   transfer_bot/src/tgbot/transfer/command/targets.rs
   看两个运行态配置页各自的业务读写和按钮布局。

7. transfer_bot/src/tgbot/transfer/command/common.rs
   看配置页共用的标题、错误卡片、导航行和 help descriptor。
```

当前运行态配置页的共通设计是：

```text
命令入口 -> callback/按钮 -> 规格表 -> 菜单输入草稿 -> 原命令复用
```

这样做的目的不是抽象本身，而是让“按钮交互”和“命令入口”最终落到同一条写库逻辑，减少后续改字段时出现按钮和命令语义漂移。

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
| `ensuring runtime database schema` | 启动期正在按当前数据库方言执行业务 migration |
| `runtime database schema ready` | 业务库 migration 已完成 |
| `runtime database state loaded` | 转存与目标运行态配置已从数据库加载或 seed 完成 |
| `ignored historical message` | 启动前历史消息被过滤 |
| `ignored non-owner message` | 非所有者消息被忽略 |
| `bot command received` | bot 收到并准备处理命令 |
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

业务数据库默认路径是 `tg/app/transfer.sqlite`，由 `storage.database_url` 控制。也支持 PostgreSQL，例如：

```text
postgresql://user:pass@127.0.0.1:5432/transfer
```

程序启动时会执行 SeaORM migration，后续表结构升级走版本化迁移。当前包含初始 schema、成功结果复用索引、动态授权表及管理员资料字段 migration。
运行时调参使用数据库中的 `transfer_runtime_config` 单行配置表；`config.json` 不再保存这些可变运行参数。
`targets` 使用 `transfer_target_config` 和 `transfer_target_alias` 两张表。旧数据库中的 `transfer_target_route` 历史表不会自动删除，但运行时不再读写，新数据库也不再创建。
动态授权使用 `authorized_user` 表，并保存可选的 Telegram 显示名称和用户名快照；启动时一次性加载权限到内存，授权面板和 `/auth add`、`/auth del` 会同时更新数据库和当前进程状态。

真实启动链会按这个顺序处理业务数据库：

```text
1. init_database_url
2. ensure_runtime_schema
3. ensure_transfer_runtime_config
4. ensure_targets_runtime_config
5. list_authorized_user_ids
```

PostgreSQL 路径同时验证表结构以及转存、目标运行态配置的首次 seed 与回读。

PostgreSQL 注意点：

- 当前运行时代码按 `current_schema()` 探测元数据。
- 测试链路会用独立 `search_path=<schema>` 创建临时 schema 验证 migration。
- 生产部署建议给应用单独数据库，或至少单独 schema。

迁移代码位置：
- 实体模型在 `transfer_bot/src/db/*.rs`
- migration 入口与版本文件在 `transfer_bot/src/db/migration/*.rs`
- 初始 schema DDL 按业务域拆在 `transfer_bot/src/db/migration/runtime_schema/*.rs`

核心表：

- `transfer_job`：主任务记录。
- `transfer_item`：任务子项。
- `transfer_result_message`：结果入口记录。
- `file_cache`：文件缓存、引用计数、删除状态。
- `menu_input_draft`：菜单输入草稿和超时信息。
- `authorized_user`：owner 动态授权的 Telegram 用户。

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

如需额外验证 PostgreSQL 路径：

```powershell
$env:TEST_POSTGRES_DATABASE_URL = "postgresql://user:pass@127.0.0.1:5432/transfer_test"
cargo test -p transfer_bot test_postgres_migration_and_insert_when_env_is_present -- --nocapture
```

这条测试会：
- 创建独立测试 schema
- 走真实启动数据库链（migration + 两类运行态 seed）
- 探测关键列
- 做一次最小插入
- 最后自动删除该 schema

最近一次完整验证结果：

- `cargo check -p transfer_bot` 通过
- `cargo test --workspace` 通过，`404 passed`
- `cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 通过

注意：运行测试会重新生成 `transfer_bot/db.test.sqlite`。如果希望工作区里不保留业务数据库，测试结束后需要手动删除它。

如果只改文档，可以不跑完整测试；如果改了 `transfer_bot/src`、配置解析或任务状态流转，建议至少跑 `test` 和 `clippy`。

## 提交前检查

```powershell
git status --short
git ls-files config.json tg/app/transfer.sqlite transfer_bot/db.sqlite transfer_bot/db.test.sqlite
```

如果 `git ls-files` 没有输出，说明这些本地状态文件没有被 Git 跟踪。
