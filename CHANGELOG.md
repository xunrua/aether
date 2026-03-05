# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

#### Phase 2 - Persona 人设模块
- 实现 `!persona list` 命令 - 列出所有可用的人设
- 实现 `!persona set <id>` 命令 - 设置当前房间的人设（需要房间管理员权限）
- 实现 `!persona off` 命令 - 关闭当前房间的人设（需要房间管理员权限）
- 实现 `!persona info <id>` 命令 - 查看人设详情
- 实现 `!persona create <id> <名称> <提示词>` 命令 - 创建自定义人设（需要房间管理员权限）
- 实现 `!persona delete <id>` 命令 - 删除自定义人设（需要房间管理员权限）
- 添加 4 个内置人设：
  - `sarcastic-dev` - 💻 毒舌程序员
  - `cyber-zen` - ☯️ 赛博禅师
  - `wiki-chan` - 📚 维基百科娘
  - `neko-chan` - 🐱 猫娘助手
- 集成 SQLite 数据库 (rusqlite)
- 创建数据库迁移系统
- 添加 `personas`、`room_persona`、`chat_history` 数据库表

### Changed

- 设置人设时自动更新 Bot 显示名称为 "人设名 (人设)"
- 关闭人设时恢复 Bot 默认名称 "Aether"
- AI 对话自动使用房间设置的人设系统提示词
- `CommandGateway` 使用 `RwLock<Parser>` 支持命令前缀运行时热更新
- 命令前缀从 `!ai` 改为 `!`

#### Phase 1 - Admin 模块
- 实现 `!bot name <名称>` 命令 - 修改 Bot 显示名称（需要 Bot 所有者权限）
- 实现 `!bot avatar <url>` 命令 - 修改 Bot 头像（需要 Bot 所有者权限，支持 PNG/JPEG/GIF/WebP）
- 实现 `!bot join <room_id>` 命令 - 加入指定房间（需要 Bot 所有者权限）
- 实现 `!bot rooms` 命令 - 列出已加入的所有房间（需要 Bot 所有者权限）
- 实现 `!bot prefix <新前缀>` 命令 - 热更新命令前缀（需要 Bot 所有者权限）
- 实现 `!bot info` 命令 - 查看 Bot 基本信息
- 实现 `!bot leave` 命令 - 离开当前房间（需要房间管理员权限）
- 实现 `!ping` 命令 - 测试响应延迟

#### Phase 0 - 基础框架
- 命令系统基础结构
  - `CommandGateway` - 命令路由核心
  - `CommandContext` - 命令执行上下文
  - `Parser` - 命令解析器（支持引号包裹参数）
  - `Permission` - 权限模型（Anyone/RoomMod/BotOwner）
  - `CommandRegistry` - 命令处理器注册表
- UI 模块 - 毛玻璃风格消息模板
  - `info_card` - 信息卡片模板
  - `help_menu` - 帮助菜单模板
  - `success/error/warning` - 状态反馈模板
- 添加 `db_path` 配置项
- 添加 `bot_owners` 配置项
- 添加 `reqwest` 和 `mime` 依赖

### Fixed

- 修复 PersonaHandler 未注册到命令系统的问题
- 修复数据库连接与 matrix-sdk 的 sqlite 依赖冲突

## [0.1.0] - 2026-03-05

### Added

- 基础项目结构
- Matrix 客户端连接与 session 持久化
- 邀请自动接受
- 消息处理（命令前缀、@提及、MSC3456 mentions）
- AI 对话（OpenAI 兼容 API）
- 流式响应（打字机效果）
- 会话历史管理（内存存储）
- 代理支持
- 单元测试框架