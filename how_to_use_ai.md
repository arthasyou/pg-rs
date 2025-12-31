# 如何使用 AI 开发本项目

将以下模板复制给 Claude Code，填写 `<>` 中的内容即可。

---

## 模板 1：新增表

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

---

## 模板 2：新增业务 API

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

---

## 模板 3：新增 HTTP 接口

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

## 模板 4：完整功能（表 + API + HTTP）

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

---

## 模板 5：修复 Bug

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

## 模板 6：修改现有 Service

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
