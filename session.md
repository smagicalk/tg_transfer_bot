# tg_transfer_bot 会话恢复文档

用于在另一台电脑或新会话中快速恢复 `tg_transfer_bot` 项目上下文。

推荐恢复提示：

```text
请先阅读 README.md 和 session.md，然后继续 tg_transfer_bot 项目。
```

## 当前状态

记录日期：2026-07-18

当前项目已经切换为单所有者模式：

- `config.json` 顶层使用单个必填 `owner_user_id`。
- 仅处理 `chat_id == sender_user_id == owner_user_id` 的 bot 私聊。
- TDLib `user` client 保留，只用于 bot 无法读取私有源时 fallback，不是交互用户。
- 普通使用者、多用户角色、ACL、积分、计费、账户余额和流水代码已经删除。
- 旧数据库不会自动 `DROP` 历史表；运行时代码不再读取旧表，新数据库也不再创建旧表。
- `transfer_job.owner_user_id` 仅作为审计字段保留，不再参与任务权限过滤。
- `/menu` 首页为：`快速转存 / 指定目标 / 任务 / 管理 / 帮助`。
- 管理页仅保留：`运行配置 / 目标配置 / 运行健康 / 文件缓存`。
- Bot 命令仅保留 `menu / transfer / lookup / downloads / job / config / health / cache / help`。
- 单所有者模式下不可达的请求 chat 路由已经删除；目标配置只保留默认目标和别名。
- 菜单旧选聊 callback、`ChatPicker` 草稿状态和共享选聊处理链已经删除。
- 配置只暴露真实可变的 `workflow.upload_client`；交互固定 bot，源读取固定 bot-first + user fallback。
- 未使用的 `utils::retry` 与纯转发 `menu/input/callbacks.rs` 已删除。
- 目标别名普通列表和搜索列表共用 `AliasListContext`，查看、删除、设默认不再各维护两套动作。
- 用户可见命令统一为完整命令，下载页不再维护或展示短命令兼容形式。
- 下载列表分页只显示当前可用的方向按钮；首页、末页和单页不再提供等价于刷新的无效导航操作。
- 手动输入目标的等待卡片支持返回目标选择，并保留原来源链接，不再需要取消后重走整个向导。
- 重复的转存中转页已经删除；帮助页可直接开始转存，历史 `m:t` 按钮也会兼容启动新向导。
- 快速转存/查询的来源输入显示为单步 `1/1`；指定目标流程仍为三步，继续输入和错误重试保持一致。
- 默认目标未配置时固定回落当前私聊；不可达的“默认目标缺失”回退分支已删除。
- 手动目标输入、继续输入和错误重试统一回显来源链接，旧 inline 等待卡也保留当前来源上下文。
- 目标选择页始终提供当前私聊；上次目标、默认目标、当前私聊和别名按 chat_id 去重，重复别名稳定保留字典序靠前项。
- 目标 callback 已删除不读取白名单的伪校验和无用 `app/config` 参数；默认目标回落当前私聊时，按钮提示同步显示“已选择当前私聊”。
- 任务页和 `/help job` 不再提供四组 job_id 手动输入按钮；统一先点状态列表，再从任务行直接查看、暂停、恢复或停止，旧 callback 继续兼容。
- 运行配置字段详情新增范围感知步进按钮，常用调整会原地刷新且无需回复数值；精确设置和恢复默认仍保留。
- 运行配置与目标配置的全量重置按钮改为危险样式“重置全部”，必须经过确认页；旧执行 callback 保持兼容。
- 任务中心移除重复“下载列表”中转入口；“查询页”改为直接启动“指定目标”查询，“任务控制”改名“更多状态”。
- 下载任务列表的返回按钮改为“任务中心”并直达任务中心，不再绕回旧下载筛选中转页；历史 `m:d` callback 仍可解析。
- 任务详情、任务列表、任务中心最近任务、实时进度、后台状态和动作结果卡中的“停止”统一使用危险样式；仍先进入确认页，最终“确认停止”同样明确标为危险操作。
- 实时进度卡统一为“任务详情/控制”优先、“列表/菜单”导航其次；等待阶段没有 job_id 时仍只展示导航，不生成空操作行。
- 查询命中进行中任务时统一为两层按钮：先展示“详情/暂停或恢复/停止”，再展示“返回列表/菜单”；停止使用危险样式，停止中状态不生成空控制行。
- 转存失败卡无论是否已创建 job，都将“重新转存”保持为主操作；没有 job_id 时仍保留失败列表和菜单导航。
- “重新转存”短回调上下文失效时，恢复卡新增“重新开始”主按钮直接进入转存来源输入，并保留菜单作为次级导航。
- 两个 GitHub Windows Workflow 已移除 job-level `env` 中不可用的 `env.*`/`runner.*` 引用；Windows 打包脚本会在缓存目录缺少 `vcpkg.exe` 时重新 clone/bootstrap。
- 转存/查询确认页新增“修改来源”，可原子返回来源输入并保留流程类型，不再需要取消后重走向导。

当前验证基线：

```text
cargo test -p transfer_bot
378 passed; 0 failed
```

交付检查已完成：`cargo fmt --all -- --check`、`cargo test -p transfer_bot`、`cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings`、`git diff --check` 以及编码/BOM 检查均通过。

## 历史记录（2026-06 多用户版本，已失效）

以下内容仅用于理解历史演进，不代表当前接口、命令、schema 或交互行为。当前行为以本文件顶部检查点和 `README.md` 为准。

记录日期：2026-06-14

更新记录：2026-06-16

- `config.json` / `config.example.json` 已进一步瘦身：文件只保留启动必需配置和 `access_control.bootstrap_admin_user_ids` 兜底管理员。
- `targets`、`access_control` 动态名单、`billing`、`transfer_config` 不再建议写在 JSON；运行时以数据库为准，通过 `/targets`、`/acl`、`/billing`、`/config` 或菜单管理。
- `BotConfig` 已单独保留 `bootstrap_admin_user_ids` 运行时字段，避免文件只保留 bootstrap 后启动 seed 丢失管理员兜底。
- `/menu` 管理页已把“目标配置 / 访问控制 / 计费配置”改成真正 callback 子页，不再只是复制命令入口。

更新记录：2026-06-17

- 启动后若 ACL 仍是空库默认状态，会向 `bootstrap_admin_user_ids` 发送“初始化引导”卡片；`targets` 是可选配置，不再阻塞菜单使用。
- `targets / acl / billing / config` 四页已升级为完整输入式管理流：按钮可直接进入 ForceReply，回复参数后复用原命令写库。
- `config` 也已补成输入式编辑流，不再只靠 `-1/+1` 调整；现支持按钮进入：
  - 设并发
  - 设删除
  - 设 GC
  - 设进度
  - 设分页
  - 设超时
- 当前四个运行态管理页按钮层级已统一为：
  - 主操作
  - 输入/调整
  - 刷新/重置/帮助
  - 菜单
  - 复制模板
- README 已补：
  - 四页管理输入流说明
  - 首启初始化引导顺序
  - 清库后如何重新配置数据库运行态
- 本轮继续补了真实数据库启动链验证：
  - `transfer_bot/src/lib.rs` 新增可复用的运行态数据库 bootstrap helper
  - 启动链现在可同时验证 migration、`transfer/billing/targets/access_control` 四类运行态 seed 和回读
  - `db::tests` 已新增 SQLite 启动链验证，并把 PostgreSQL 测试升级为走同一条真实启动链
- 本轮继续收口四个运行态管理页：
  - `config / targets / acl / billing` 的页头说明与帮助/菜单导航行已抽到公共 helper
  - `billing` 已补齐数值输入入口：`设基础 / 设单项 / 设初始 / 设公告`
  - 四页按钮层级进一步统一为“主操作 -> 输入/调整 -> 帮助/菜单 -> 复制”
- 本轮新增可观察日志点：
  - `ensuring runtime database schema`
  - `runtime database schema ready`
  - `runtime database state loaded`

更新记录：2026-06-18

- README 和 session 已补充当前开发阅读顺序，方便后续接手时直接从命令总线、菜单输入和四个运行态管理页开始看。
- 四个运行态管理页的实现现在更明确地是“规格表驱动”：
  - `ConfigFieldSpec`
  - `TargetsInputSpec`
  - `AclInputSpec`
  - `BillingNumericSpec` / `BillingAnnouncementSpec`
- 菜单输入分发已经不再在主流程里硬编码四套 action 分类，而是先按规格反查命令模块，再统一进入原命令入口。
- `AdminInputAction::ALL` 已补测试专用全量覆盖，新增管理输入动作时更容易发现只加枚举、没接规格的遗漏。
- `common.rs` 里已新增统一的运行态管理页错误卡片 helper，四页 callback 错误和编辑失败提示开始完全同源。

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

- `/menu` callback 入口已抽出纯决策层，先统一处理 TDLib payload 类型、菜单 payload 解析和输入类按钮归属校验，再进入实际副作用分支。
- `menu/input/callbacks_target.rs` 已继续收口目标选择流程：目标 callback 公共参数集中到 `TargetCallbackContext`，默认目标和常用目标复用同一套“推进草稿到确认页”逻辑。
- 确认按钮消费结果已映射为 `ConfirmCallbackDecision`，过期、空草稿、错误阶段和正常执行的 UI 行为更容易单测和维护。
- `menu/input.rs` 已补 `ContinueInputDecision`，把“无草稿 / 过期 / 活跃草稿”的继续输入分支统一成纯决策。
- `callbacks_target.rs` 已补 `TargetAdvanceCallbackDecision`，把目标推进按钮的过期、空草稿和错误阶段提示集中到一处。
- `workflow/recovery.rs` 已补启动恢复摘要聚合测试，确认会按 `request_chat_id` 聚合并限制示例 job 数量。
- `menu.rs` 已补 `MenuCallbackRoute`，把 `/menu` callback 入口拆成“页面路由 / 启动输入 / 继续输入 / 转发到具体 handler”四类分发。
- `menu.rs` 测试里已补入口高层计划层 `MenuCallbackPlan`，用于稳定描述 `/menu` callback 的最终意图，而不直接绑定 TDLib 发送细节。
- “继续输入为空”和“继续输入过期”现在都有独立文案构造函数，交互闭环的终态提示已统一成恢复态/空态，而不是继续复用等待态。
- `workflow/recovery.rs` 已补“何时应该发送恢复摘要”测试，明确它只服务于转存任务恢复，不包含菜单草稿这种懒恢复状态。
- `flow_callbacks.rs` 已补共享选聊和目标文本输入的纯决策锚点：`SharedChatOutcome`、`TargetInputOutcome`，并把旧选聊兼容主流程接回这些高层分支。
- 交互文案继续统一：旧选聊目标失败的“目标不可用”提示已抽成单点，首页/继续输入的空态提示也继续向统一卡片语言收敛。
- `flow_callbacks.rs` 已继续补 `SourceLinkOutcome`，把“源链接非法 / 默认目标不可用 / 进入目标选择 / 直接走默认目标执行”统一成高层动作。
- `target.rs` 已支持带附加说明的目标选择卡片，默认目标不可用与旧选聊失败回退现在会保留来源上下文并回到同一种目标选择视图。
- `menu/text.rs` 已补统一的 `build_menu_target_unavailable_text` 与 `build_menu_no_pending_input_text`，目标不可用和无未完成输入都收口到统一终态卡片语言。
- `flow_callbacks.rs` 已补旧选聊 ignored/wrong-step 边界测试，以及默认目标不可用回退路径的高层动作测试；目标选择链路的交互回退语义现在更稳。
- `flow_callbacks.rs` 已继续收口成更统一的“阶段动作 -> UI 动作”模型：`SourceLinkOutcome`、`TargetInputOutcome`、`SharedChatOutcome` 现在都会进一步映射到统一的 `FlowUiAction`。
- 当前 `flow_callbacks.rs` 的角色已经更接近纯编排层：先决定动作，再执行 UI；后续如果还要继续优化交互，这里已经是稳定的继续演进点。
- 跨交互页的页头/命令分区/空态说明 helper 已下沉到 `command/common.rs`，`downloads`、`cache`、`points` 三页已开始统一到同一套“ready + 命令 + 空态”风格。
- `job`、`health`、`help` 三页也已接入同一套公共 helper，菜单外的主要交互页现在基本共享统一的页头、命令区和说明块风格。
- 按钮层级也开始统一：`job`、`health`、`points`、`cache`、`help` 等页正收口到“主操作 / 刷新返回菜单 / 复制动作”更稳定的按钮层次。
- 本轮继续把按钮最密的三页再收一轮：
  - `downloads` 现已统一为“任务/筛选主操作 -> 刷新/返回/菜单 -> 复制当前命令 -> 分页单独一行”。
  - `config` 现已统一为“配置增减主操作 -> 刷新/返回/菜单 -> 复制命令”，并补了层级测试。
  - `menu` 的 `Home / Transfer / Downloads / Jobs / Lookup / Help / Config fallback` 已继续拉平按钮密度和导航层级。

更新记录：2026-06-19

- README、`/help` 和 `/menu` 文案已同步收口“最终对外可开放能力”的边界：
  - 普通用户建议只开放：
    - `/help`
    - `/menu`
    - `/transfer`
    - `/lookup`
    - `/downloads`
    - `/job`
    - `/balance`
  - admin 额外使用：
    - `/points`
    - `/config`
    - `/targets`
    - `/acl`
    - `/billing`
    - `/health`
    - `/cache`
- README 的“最小可运行配置”已修正为只保留启动级文件配置：
  - `bootstrap_admin_user_ids` 仍在 `config.json`
  - `targets / acl / billing / transfer runtime config` 以数据库运行态为准，不再要求先写入文件
- README 已补首启建议：
  1. 先用 bootstrap admin 私聊 bot 打开 `/menu`
  2. 先确认 `acl` 的普通用户入口策略和目标白名单
  3. 再按需配置 `targets`
  4. 最后按需调 `billing / config`
- 现在已明确：`targets` 可选，不配置时默认目标回落到当前私聊；普通用户开放前主要检查 `acl` 策略。
- `targets` 列表页已继续收紧为“先选对象，再选动作”：路由列表和别名列表只显示编号按钮，进入详情后再改目标、设默认或删除；详情页返回会回到来源列表或来源搜索结果。
- `acl` 列表页也已收紧为同一套“编号 -> 详情 -> 删除/解除”流程：管理员、允许用户、封禁、目标白名单和请求白名单列表不再直接显示 `删1`，详情页返回会回到来源列表。
- `help transfer / lookup / downloads / job / menu` 与菜单各页文案已补上权限边界：
  - 普通用户只看自己的任务
  - 普通用户只查自己的结果
  - 普通用户只能控制自己的任务
  - 普通用户不能借 `user` fallback 读取私有源
- “默认目标”的语义已在 README 中明确：
  - 当前按 `targets.by_request_chat_id[request_chat_id]` 做私聊隔离
  - 在 bot 私聊模式下，这基本等价于“每个用户自己的默认目标”
  - 暂不需要为此单独引入 `by_user_id` 语义
- 本轮验证已通过：
  - `cargo fmt --all`
  - `cargo test -p transfer_bot`
  - `cargo check -p transfer_bot`
  - `cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings`
  - 当前测试数：`460 passed`
- 本轮继续完成第二批交互页统一：
  - `cache` 现已统一为“视图主操作 -> 刷新/健康/菜单 -> 分页单独一行”，常规按钮区不再保留复制命令。
  - `points` 流水页现已统一为“分页主操作 -> 刷新/返回/菜单 -> 复制当前命令 -> 复制余额”，admin 返回按钮会安全降级为复制 `/points show <user_id>`。
  - `health` 现已统一为“主操作 -> 刷新/帮助/菜单”，常规按钮区不再保留复制 `/health`。
  - `help detail` 现已统一成纯导航结构，按钮区只保留真实入口与“返回目录 / 菜单”。
- 本轮继续把 `help index` 目录页彻底按同一套层级重排：
  - 主导航先放公开 help topic。
  - admin 专属 topic 独立成自己的主操作行，不再和复制按钮混排。
  - `刷新 / 帮助说明 / 菜单` 固定成单独导航行。
  - `help` 目录页不再保留复制命令按钮，固定只保留 `刷新 / 帮助说明 / 菜单` 这一条 footer 导航。
- 本轮继续收缩“非必要复制按钮”：
  - `tgbot/error.rs` 里的命令错误卡片已修正为“按钮文案和真实行为一致”，不再统一错误地跳菜单。
  - `menu` 中 `transfer / downloads / jobs / lookup` 这几页的模板复制按钮已删掉一批，改为优先使用现有交互入口。
  - `lookup`、转存进度卡片、成功结果卡片、失败卡片和中间状态卡片已继续收缩按钮区：能走 callback 的统一改成“查看任务详情 / 查看列表 / 菜单”，不再把“复制查询命令 / 复制重新转存”放在按钮区重复表达。
  - `transfer` 首次回执、`job pause` 结果卡片和启动恢复摘要也已收掉残留动作型复制按钮，改为直接 callback 导航；按钮区只保留源标识、`job_id` 这类排查数据复制。
  - `help index` 已去掉 `/health`、`/config reset`、`/config show`、`/cache` 这类重复复制入口；这些页面都有真实 topic callback。
  - `/help health` 与 `/help cache` 详情页也已去掉复制命令按钮，只保留打开页面、关联页面、返回目录和菜单。
  - `/help job`、`/help downloads`、`/help points` 已去掉动作复制模板，按钮区只保留输入流或真实 callback，命令示例继续留在正文。
  - `/help` 自身详情页以及 `/help config|targets|acl|billing` 也已改成纯导航，不再在按钮区额外挂 `复制 /help` 或 `复制 /config show` 这类 show 命令。
  - `/help transfer` 与 `/help lookup` 也已去掉 `复制示例`，按钮区只保留真实交互入口，示例统一留在正文。
  - 所有普通 UI 的 `停止` 按钮现在先发 `j:sc:<job_id>` 打开确认页；确认页里的 `确认停止` 才发旧的 `j:s:<job_id>` 真正停止，因此历史按钮仍兼容。
  - `menu/input.rs`、`menu/input/flow_callbacks.rs`、`menu/text.rs` 本轮继续把几处 `unreachable!` 风险改成可恢复分支：即使未来状态机分支意外漂移，也会回到“重问源链接 / 重问目标 / 返回菜单”这类恢复路径，而不是直接 panic。
  - 本轮继续清理“同页重复功能”按钮：
    - `menu/transfer` 页移除了与首行完全等价的 `指定目标 / 默认目标`。
    - `help index` 页移除了重复的 `帮助说明` 入口，只保留 footer 那一个。
    - `points ledger` 页移除了与页码中间按钮重复的 `复制当前命令` 行。
    - `menu/home` footer 移除了首页自指的 `首页` 按钮，只保留 `刷新 / 帮助`。
    - `menu/help` 页移除了不够统一的 `复制帮助命令`，帮助页只保留 topic callback 与页尾导航。
    - `user config fallback` 的 footer 不再自指首页，改成 `刷新 / 帮助 / 返回`。
  - 本轮再次复核后，`user config fallback` 与 `help index` 的重复导航也已进一步收口，按钮页尾现在更偏向单一返回路径而不是多处同义入口。
  - 本轮开始把菜单重排为多级结构：
    - `MenuPage` 新增 `TasksHub / AccountHub / AdminHub` 三个二级 hub。
    - 首页现在只保留高频动作和 hub 入口，不再直接承载所有细页导航。
    - 原首页关于状态快捷、账户入口、管理入口的测试也已迁移到各自 hub，菜单测试已和新结构对齐。
  - 本轮继续把首页彻底“去任务化”：`recent_job_buttons` 与最近任务快捷控制已下沉到 `TasksHub`，首页不再同时承担任务面板职责。
  - 本轮继续打磨多级菜单：
    - `TasksHub` 现在承接最近任务列表、状态快捷、任务控制与查询入口。
    - `AccountHub` 现在承接余额、积分流水，admin 额外展示用户流水。
    - `AdminHub` 现在承接运行配置、健康、缓存、用户流水，并补了普通用户的权限拒绝页测试。
    - 本轮继续收口：
      - 首页已彻底去任务化，只保留高频动作、hub 导航和 footer。
      - 最近任务详情与快捷控制已稳定下沉到 `TasksHub`。
      - 首页文案已改成 hub 语义，不再暗示首页直接承担任务面板职责。
  - 当前仍保留的复制按钮主要集中在：错误详情、结果链接/定位、`job_id`、`/cancel`、未命中 lookup 的转存命令、失败后的重试命令等必须由用户主动发送或复制的数据。
- 本轮只新增了一个最小公共 helper：`build_return_menu_row`，用于那些没有合理刷新语义的页面，不继续机械抽象更多 helper。
- 当前判断：下一处若还要继续优化交互，应优先看 `flow_callbacks.rs` 的高层状态表达，不建议先硬拆 `input/state.rs`。
- 新增菜单 callback 决策测试、目标 callback 上下文测试和确认按钮决策测试，避免后续交互文案调整破坏流程语义。
- 命令错误提示和权限/自动转存引导已下沉到 `transfer_bot/src/tgbot/error.rs`。
- 交互回调错误卡片发送已下沉到 `transfer_bot/src/tgbot/send/error.rs`。
- `transfer_bot/src/tgbot/transfer/command/common.rs` 只保留命令公共拼装逻辑。
- `help`、`downloads`、`cache`、`menu`、`points`、`config_cmd`、`job` 的 callback 错误卡片统一走同一发送入口。
- 新增发送失败日志，便于排查 callback 错误卡片本身的发送问题。
- `transfer_bot/src/tgbot/send.rs` 现在只保留发送层入口转发，错误卡片排版细节已拆出。
- `/balance` 与 `/points` 已拆成命令入口、流水渲染和 callback 子模块。
- 新增恢复/查重专项测试：成功结果复用、重复活跃任务、同请求取消幂等、启动恢复扫描。
- 最新验证：`cargo fmt --all`、`cargo check -p transfer_bot`、`cargo test -p transfer_bot`、`cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 全部通过，`cargo test -p transfer_bot` 当前 `460 passed`。
- 最新补充验证：
  - `cargo test -p transfer_bot db::tests -- --nocapture` 通过，当前 `8 passed`
  - `cargo test -p transfer_bot billing::tests -- --nocapture` 通过
  - `cargo test -p transfer_bot config_cmd::callback::tests -- --nocapture` 通过
  - `cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 通过

当前未提交改动较多，具体以 `git status --short` 为准；主要集中在：

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

- `tdlib_rs` 是生成代码目录，不要手动清理或重排，除非明确要重新生成绑定。
- `config.json` 是本地敏感配置，已忽略，不要提交。
- 当前同时在推进两条线：`AppContext` 架构重构，以及 bot 菜单/交互一致性优化。
- 当前已经加入 admin/普通用户权限隔离和普通用户积分计费。
- 独立 `migration` workspace crate 仍然不保留，但业务库已恢复为 `transfer_bot/src/db/migration/` 下的内置版本化迁移。
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
- admin 只能私聊 bot 交互，可全局查看和控制任务，不扣积分，可使用 user fallback。
- 普通用户只能私聊 bot，只能查看和控制自己的任务，链接源不允许借 user fallback。
- 普通用户对外建议只开放 `/help /menu /transfer /lookup /downloads /job /balance`。
- `targets` 不再要求初始化；普通用户开放前主要确认 `acl` 的入口策略和目标白名单。
- 普通用户转存按 `billing.base_cost_points + billing.item_cost_points * item_count` 扣积分。
- 扣费发生在 spider 成功后、创建 job 前；无效链接不会扣费。
- 积分账本使用 `request_chat_id + request_message_id` 生成幂等键，防止同一命令重复扣费。
- 任务全部失败或用户停止时会全额退款，并通过 `billing_status = charged -> refunded` 保证幂等。
- 部分成功会按失败条目占比退款；只要存在失败且扣过费，至少退 1 分，最多不超过本次扣费。
- 业务数据库不再使用独立 workspace `migration` crate。
- 当前启动时会执行 `transfer_bot/src/db/migration/` 下的 SeaORM 版本化迁移。
- 启动时直接 `db::ensure_runtime_schema(...)` 建当前完整表结构。
- 开发期允许直接删除业务库，程序下次启动会自动重建。
- 重复转存判断固定看 `source_link + target_chat_id`。
- 请求级幂等固定看 `request_chat_id + request_message_id`。
- 私聊模式下 `targets.by_request_chat_id[request_chat_id]` 基本等价于“该用户自己的默认目标”。
- 文件缓存保留，消息缓存不做独立表。
- 菜单输入草稿持久化在 `menu_input_draft`，程序重启后未完成交互仍可继续。
- 项目不支持群聊命令交互；目标群只作为转存目的地，通过私聊菜单或命令参数选择。

## 当前交互流程

### 首页

`/menu` 打开首页。

首页内容：

- 运行摘要：活跃任务、失败任务、待恢复、待删缓存、删失败、最近任务数
- 直达动作：`开始转存`、`快速转存`、`快速查询`
- 状态直达：`运行任务`、`失败任务`、`已暂停`
- 其他入口：`下载列表`、`任务控制`、`转存页`、`查询页`、`运行配置`、`帮助`、`运行健康`、`文件缓存`、`用户流水`
- 最近任务快捷按钮：运行中可直接 `暂停`，暂停态可 `恢复`；`停止` 会先打开确认页
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
- 当前私聊或显式默认目标
- 配置别名
- 手动输入 `chat_id / alias / default`

`快速转存` 会优先用默认目标；如果没有显式默认目标，会直接回落到当前私聊。只有默认目标被 `allowed_target_chat_ids` 拦截时，才会退回普通选目标流程。

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
- 单任务直控：运行中 `暂停`，暂停态 `恢复`；`停止` 会先打开确认页

### 输入流程行为

- 任意输入流程可发 `/cancel`
- reply keyboard 场景也支持 `取消` / `cancel`
- 从菜单点进输入向导时，原消息会被编辑成等待态，只保留 `取消` / `首页`
- 如果用户在输入流程里直接发送新命令，旧草稿会被丢弃
- 如果旧草稿停在原生选聊阶段，会自动收起旧键盘
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

- `/balance`：查看当前用户积分余额。
- `/balance history [limit] [page]`：查看当前用户积分流水。
- `/points show <user_id>`：admin 查看指定用户积分。
- `/points history <user_id> [limit] [page]`：admin 查看指定用户积分流水。
- `/points add <user_id> <amount> [reason]`：admin 加分。
- `/points sub <user_id> <amount> [reason]`：admin 扣分。

本轮已修复：

- `/help points` 现在能正常展开，不再是余额卡片里的死链接。
- `/menu -> 帮助` 已加入积分帮助入口。
- admin 请求 chat 判断不再复用合并后的 `admin_ids`，且所有交互入口都只接受私聊 bot。
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

- `cargo fmt --all` 通过
- `cargo check -p transfer_bot` 通过
- `cargo test -p transfer_bot` 通过，`460 passed`
- `cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings` 通过

额外验证：

- `cargo test -p transfer_bot db::tests -- --nocapture` 通过
- `cargo test -p transfer_bot test_postgres_migration_and_insert_when_env_is_present -- --nocapture` 在设置 `TEST_POSTGRES_DATABASE_URL` 时通过

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
