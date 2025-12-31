# pg-rs

一个为 **AI 驱动开发** 设计的 Rust 分层应用框架。

> **设计理念**：本项目的目标是让 AI（Claude Code / Codex / Cursor）来编写业务代码，人类只需描述需求。

## 核心特点

- **AI 友好架构**：5 层分离，每层职责明确，AI 容易理解和遵守
- **协议驱动**：`ai_protocols/` 目录包含 AI 执行规约，确保代码质量
- **即用模板**：`how_to_use_ai.md` 提供复制即用的提示词模板
- **零手写代码**：理想情况下，人类只需要描述需求，AI 完成所有编码

## 快速开始（AI 开发模式）

### 1. 准备环境

```bash
# 启动 PostgreSQL
make postgres

# 运行迁移
make migrate-up
```

### 2. 让 AI 开发功能

打开 Claude Code / Cursor / Codex，复制以下内容：

```
请先阅读 ai_protocols/TABLE_ADDING_PROTOCOL.md 了解项目架构规范。

然后帮我实现以下功能：
<描述你的需求>
```

或者使用 `how_to_use_ai.md` 中的模板。

### 3. 运行验证

```bash
cargo check
cargo run -p web-server
```

## 架构概览

```
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  Layer 5: HTTP API
├──────────────────────────────────────────────────────┤
│                     demo-db                          │  Layer 4: 业务 API（跨表编排）
├──────────────────────────────────────────────────────┤
│                    pg-tables                         │  Layer 3: 领域服务（单表操作）
├──────────────────────────────────────────────────────┤
│                     pg-core                          │  Layer 2: 基础设施（不可修改）
├──────────────────────────────────────────────────────┤
│                    migration                         │  Layer 1: 数据库迁移
└──────────────────────────────────────────────────────┘
```

### 层级职责

| 层级 | Crate | 职责 | AI 可修改 |
|------|-------|------|----------|
| 5 | web-server | HTTP 路由、Handler、请求/响应 DTO | 可以 |
| 4 | demo-db | 业务 API，编排多个 Service | 可以 |
| 3 | pg-tables | 单表 Service、DTO、Entity | 可以 |
| 2 | pg-core | 连接池、错误处理、Repository trait | 禁止 |
| 1 | migration | 数据库表结构 | 可以 |

## AI 开发文档

| 文件 | 用途 |
|------|------|
| `ai_protocols/TABLE_ADDING_PROTOCOL.md` | AI 执行规约（必读） |
| `how_to_use_ai.md` | 提示词模板（复制即用） |
| `PROJECT_DESIGN_EVALUATION.md` | 项目设计评审 |

## 示例：让 AI 添加新功能

### 场景：添加用户表和接口

复制以下内容给 Claude Code：

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要实现完整功能：

### 1. 新增表
- 表名：user
- 字段：
  - user_id: bigint (主键, 自增)
  - username: varchar(255) (唯一)
  - email: varchar(255)
  - created_at: timestamp

### 2. 业务 API
- 功能：用户注册和查询
- 输入：用户名、邮箱
- 输出：用户信息

### 3. HTTP 接口
- POST /users - 创建用户
- GET /users/{id} - 查询用户

按 5 层架构顺序完成：
1. migration → 建表
2. 提示我运行脚本生成 entity
3. pg-tables → dto + service
4. demo-db → api + dto
5. web-server → handler + route + dto

禁止修改 pg-core。
```

AI 会自动：
1. 创建 migration 文件
2. 在 pg-tables 中生成 DTO 和 Service
3. 在 demo-db 中创建业务 API
4. 在 web-server 中添加 HTTP 接口

## 数据模型

当前示例是健康数据管理系统：

```
┌──────────┐     ┌─────────────┐     ┌────────────┐
│ subject  │────▶│ observation │◀────│   metric   │
└──────────┘     └─────────────┘     └────────────┘
                        │
                        ▼
                 ┌─────────────┐
                 │ data_source │
                 └─────────────┘
```

- **subject** - 观测主体（人、设备）
- **metric** - 指标定义（身高、血压、血糖等）
- **observation** - 观测记录（事实表）
- **data_source** - 数据来源（手工、设备、报告）

## API 接口

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/medical/observations` | 查询观测数据 |
| POST | `/medical/observations` | 记录观测数据（含来源） |

Swagger UI: http://localhost:19878/swagger-ui

## 开发命令

```bash
make help              # 查看所有命令
make postgres          # 启动 PostgreSQL
make migrate-up        # 运行迁移
make migrate-fresh     # 重建数据库
make build             # 编译项目
make test              # 运行测试
```

## 技术栈

| 组件 | 技术 |
|------|------|
| 运行时 | Tokio |
| ORM | SeaORM 2.0 |
| Web 框架 | Axum 0.8 |
| OpenAPI | utoipa + Swagger UI |
| 序列化 | serde |
| 日志 | tracing |

## 为什么选择 AI 驱动开发？

1. **一致性**：AI 严格遵守协议，代码风格统一
2. **效率**：描述需求比手写代码快 10 倍
3. **质量**：协议约束避免常见错误（跨层调用、错误处理等）
4. **可维护**：分层架构清晰，新人（包括 AI）容易上手

## License

MIT or Apache-2.0
