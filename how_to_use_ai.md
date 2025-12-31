# AI 驱动开发指南

本项目专为 **AI 编程助手**（Claude Code / Cursor / Codex / GitHub Copilot）设计。

> **核心理念**：人类描述需求，AI 编写代码。

---

## 快速开始

### 方式 1：Claude Code

```bash
# 在项目目录启动 Claude Code
claude

# 然后输入你的需求，例如：
> 请先阅读 ai_protocols/TABLE_ADDING_PROTOCOL.md，然后帮我添加一个用户表
```

### 方式 2：Cursor

1. 用 Cursor 打开项目
2. 按 `Cmd+K` 打开 AI 对话
3. 粘贴下方模板

### 方式 3：其他 AI 工具

1. 将 `ai_protocols/TABLE_ADDING_PROTOCOL.md` 内容复制给 AI
2. 描述你的需求
3. AI 会按照协议生成代码

---

## 首次使用（必读）

在让 AI 开发功能之前，先让它阅读协议：

```
请阅读 ai_protocols/TABLE_ADDING_PROTOCOL.md 文件，了解本项目的架构规范。
阅读完成后，告诉我你理解的要点。
```

AI 应该能总结出：
- 5 层架构及各层职责
- pg-core 不可修改
- 单表原则（pg-tables 层）
- 跨表编排在 demo-db 层

---

## 提示词模板

### 模板 1：新增数据表

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要新增一张表：
- 表名：<table_name>
- 表语义：<一句话描述这张表存储什么事实>
- 字段：
  - <field1>: <type> <约束>
  - <field2>: <type> <约束>
  - ...

完成以下工作：
1. migration 建表
2. 提示我运行 ./scripts/fresh_db.sh 生成 entity
3. pg-tables 中创建 dto 和 service

禁止修改 pg-core。
```

**示例**：

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要新增一张表：
- 表名：device
- 表语义：记录医疗设备信息
- 字段：
  - device_id: bigint (主键, 自增)
  - device_code: varchar(100) (唯一, 设备编号)
  - device_name: varchar(255) (设备名称)
  - device_type: varchar(50) (设备类型)
  - status: varchar(20) (状态: active/inactive)
  - created_at: timestamp

完成以下工作：
1. migration 建表
2. 提示我运行 ./scripts/fresh_db.sh 生成 entity
3. pg-tables 中创建 dto 和 service

禁止修改 pg-core。
```

---

### 模板 2：新增业务 API

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要在 demo-db 中新增业务 API：
- API 名称：<ApiName>
- 功能描述：<描述这个 API 做什么>
- 涉及的表：<table1>, <table2>, ...
- 输入参数：<参数列表>
- 返回数据：<返回内容描述>

在 demo-db 层完成：
1. dto/<domain>.rs - 定义请求和响应结构
2. api/<domain>.rs - 实现 API，编排多个 Service

禁止修改 pg-core。
禁止在 API 中直接操作数据库。
```

**示例**：

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要在 demo-db 中新增业务 API：
- API 名称：DeviceApi
- 功能描述：设备管理，包括注册设备和查询设备绑定的观测数据
- 涉及的表：device, observation
- 输入参数：device_code, subject_id
- 返回数据：设备信息 + 该设备记录的最近观测

在 demo-db 层完成：
1. dto/device.rs - 定义请求和响应结构
2. api/device.rs - 实现 API，编排 DeviceService 和 ObservationService

禁止修改 pg-core。
禁止在 API 中直接操作数据库。
```

---

### 模板 3：新增 HTTP 接口

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要在 web-server 中新增 HTTP 接口：
- 路径：<HTTP method> /<path>
- 功能：<描述>
- 请求参数：<Query/Body 参数>
- 响应格式：<响应结构>
- 调用的 API：<demo-db 中的 API>

在 web-server 层完成：
1. dto/<domain>.rs - 请求/响应 DTO
2. handlers/<domain>.rs - Handler 实现
3. routes/<domain>.rs - 路由注册
4. 添加 OpenAPI 注解

禁止修改 pg-core。
禁止在 Handler 中直接使用 pg-tables Service。
```

---

### 模板 4：完整功能（表 + API + HTTP）

这是最常用的模板，一次性完成整个功能：

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要实现完整功能：

### 1. 新增表
- 表名：<table_name>
- 字段：
  - <field1>: <type>
  - <field2>: <type>

### 2. 业务 API
- 功能：<描述>
- 输入：<参数>
- 输出：<返回>

### 3. HTTP 接口
- <METHOD> /<path> - <描述>

按 5 层架构顺序完成：
1. migration → 建表
2. 提示我运行脚本生成 entity
3. pg-tables → dto + service
4. demo-db → api + dto
5. web-server → handler + route + dto

禁止修改 pg-core。
```

**示例**：

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要实现完整功能：

### 1. 新增表
- 表名：prescription
- 字段：
  - prescription_id: bigint (主键)
  - subject_id: bigint (患者)
  - doctor_name: varchar(100)
  - medication: text (药品信息 JSON)
  - prescribed_at: timestamp
  - created_at: timestamp

### 2. 业务 API
- 功能：开具处方、查询患者处方历史
- 输入：subject_id, doctor_name, medication
- 输出：处方详情、处方列表

### 3. HTTP 接口
- POST /prescriptions - 创建处方
- GET /prescriptions?subject_id=xxx - 查询处方

按 5 层架构顺序完成：
1. migration → 建表
2. 提示我运行脚本生成 entity
3. pg-tables → dto + service
4. demo-db → api + dto
5. web-server → handler + route + dto

禁止修改 pg-core。
```

---

### 模板 5：修复 Bug

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 的架构边界执行。

Bug 描述：<描述问题>
复现步骤：<如何复现>
期望行为：<应该是什么样>

修复时遵守：
- 不改变现有架构
- 不引入新的抽象
- 禁止修改 pg-core
```

---

### 模板 6：修改现有 Service

```
请严格按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行。

需要修改 pg-tables 中的 <ServiceName>：
- 修改内容：<描述要改什么>
- 原因：<为什么要改>

遵守单表原则：
- 不跨表 JOIN
- 不校验外键存在
- 禁止修改 pg-core
```

---

## 工作流程

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  1. 描述需求  │────▶│  2. AI 生成  │────▶│ 3. 运行验证  │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │
       ▼                   ▼                   ▼
   填写模板            按协议编码           cargo check
   复制给 AI          修改多个文件          cargo run
```

### 验证命令

```bash
# 编译检查
cargo check

# 运行服务
cargo run -p web-server

# 查看 API 文档
open http://localhost:19878/swagger-ui
```

---

## 常见问题

### Q: AI 生成的代码编译不过？

A: 让 AI 修复：
```
cargo check 报错如下：
<粘贴错误信息>

请按照 ai_protocols/TABLE_ADDING_PROTOCOL.md 修复。
```

### Q: AI 修改了 pg-core？

A: 明确告诉它：
```
你刚才修改了 pg-core，这是禁止的。
请撤销对 pg-core 的修改，按照协议重新实现。
```

### Q: 需要跨表查询怎么办？

A: 跨表逻辑放在 demo-db 层：
```
需要在 demo-db 层实现跨表查询：
- 查询 subject 信息
- 同时查询该 subject 的所有 observations
- 聚合返回

禁止在 pg-tables 层跨表 JOIN。
```

### Q: Entity 生成后字段不对？

A: 检查 migration，然后重新生成：
```bash
# 修改 migration 后
./scripts/fresh_db.sh
```

---

## 进阶用法

### 批量操作

```
请按顺序完成以下任务，每个任务按 ai_protocols/TABLE_ADDING_PROTOCOL.md 执行：

1. 新增 notification 表（通知记录）
2. 新增 NotificationService
3. 在 HealthApi 中添加发送通知的方法
4. 添加 POST /notifications 接口

完成一个任务后，告诉我，我确认后继续下一个。
```

### 代码审查

```
请审查以下文件是否符合 ai_protocols/TABLE_ADDING_PROTOCOL.md：
- crates/pg-tables/src/table/xxx/service.rs
- crates/demo-db/src/api/xxx.rs

检查：
1. 是否有跨层调用
2. 是否修改了 pg-core
3. 错误处理是否正确
```

---

## 最佳实践

1. **先让 AI 读协议**：首次使用时让 AI 阅读并总结协议
2. **一次一个功能**：避免一次性让 AI 做太多事情
3. **及时验证**：每完成一个步骤就 `cargo check`
4. **保留上下文**：在同一个对话中完成相关功能
5. **明确约束**：每次都强调"禁止修改 pg-core"
