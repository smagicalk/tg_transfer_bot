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
| `tdlib_config.api_id` | Telegram API ID |
| `tdlib_config.api_hash` | Telegram API Hash |
| `tdlib_config.database_directory` | TDLib 数据库目录 |
| `tdlib_config.files_directory` | TDLib 文件目录 |
| `tdlib_config.database_encryption_key` | TDLib 本地数据库加密 key |
| `admin_ids` | 允许使用机器人的 user/chat id |
| `target_map` | 源 chat 到目标 chat 的映射，`0` 可作为兜底目标 |
| `transfer_config.job_concurrency` | 后台转存任务并发数 |
| `transfer_config.file_delete_delay_hours` | 文件引用归零后的延迟删除小时数 |
| `transfer_config.file_gc_interval_seconds` | 文件删除队列扫描间隔秒数 |
| `login_info` | 登录方式，支持 `OCR`、`PHONE`、`TOKEN` |

`config.json`、`tg/`、SQLite 数据库和日志文件都是本地运行状态，不应该上传到 GitHub。

## 运行

在仓库根目录运行：

```powershell
$env:LOCAL_TDLIB_PATH = "F:/tdlib/td/tdlib"
cargo run -p transfer_bot -- -c config.json
```

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
| `/transfer <link> [target_chat_id]` | `/t <link> [target_chat_id]` | 创建转存任务 |
| `/lookup <link> [target_chat_id]` | `/lk <link> [target_chat_id]` | 查询历史转存结果 |
| `/downloads [filter] [limit] [page]` | `/d [filter] [limit] [page]` | 查询任务列表和下载进度 |
| `/job <pause|resume|stop> <job_id>` | `/j <p|r|s> <job_id>` | 手动控制任务 |
| `/config [show|set <key> <value>]` | `/cfg [show|set <key> <value>]` | 查看或修改运行配置 |

### 转存

```text
/t https://t.me/c/123/456
/transfer https://t.me/c/123/456 -1001234567890
```

不传 `target_chat_id` 时，会按 `target_map` 查找目标聊天；找不到源 chat 的映射时，会尝试使用 `target_map` 中的 `0`。

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
/cfg set file_delete_delay_hours 3
/cfg set file_gc_interval_seconds 30
```

只有转存运行参数支持动态修改。TDLib 登录、API ID、API Hash 等启动级配置不通过机器人命令修改。

## 任务流程

1. 管理员发送 `/transfer <link> [target_chat_id]`。
2. 机器人解析源 chat、源 message 和目标 chat。
3. 任务先写入数据库，避免进程退出后丢失状态。
4. 后台执行器拉取源消息或相册，并为每个媒体项建立 `transfer_item`。
5. 文件通过 `file_cache` 去重下载，相同文件只保留一个下载任务。
6. 所有文件准备完成后，按原顺序上传到目标聊天，尽量组成 album。
7. 上传成功后记录结果消息和链接。
8. 任务完成或停止后释放文件引用；引用为 0 的文件进入延迟删除队列。
9. 程序下次启动时会恢复未完成任务。

## 数据库说明

项目使用 SeaORM migration 创建表结构，启动时会执行迁移。

核心表：

- `transfer_job`：一次转存请求的主任务，记录源链接、目标 chat、状态和转存结果。
- `transfer_item`：任务中的单个消息或媒体项，记录源消息、文件 key 和子状态。
- `file_cache`：本地文件缓存与引用计数，负责下载去重和延迟删除。

本地 SQLite 数据库属于运行状态，已被忽略，不要提交。

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
git ls-files config.json transfer_bot/db.sqlite transfer_bot/db.test.sqlite
```

`git ls-files` 没有输出时，说明这些本地状态文件没有被 Git 跟踪。
