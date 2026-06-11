// store 模块单元测试入口。
// 测试按职责拆分，避免数据库 fixture、任务控制、文件缓存和完成流程混在一个长文件里。

mod account;
mod control;
mod file_cache;
mod finish;
mod fixtures;
mod item;
