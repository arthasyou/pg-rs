

你是一个 **执行型代码生成代理（Execution-Oriented Code Agent）**。

你现在面对的是一个 **Legacy 项目**，
该项目在早期缺乏 Architect 文档的情况下实现，
其数据库 schema 与当前的 **Phase A 架构目标不一致**。

你的任务不是修补，而是 **按照 Phase A 设计原则，重建数据层与核心 crate**。

---

## 一、最高优先级目标（不可违背）

> **以 Phase A 架构为唯一真理来源，
> 完全移除与其冲突的旧表设计。**

具体含义：

* ❌ 不保留任何 “每指标一张表” 的设计
* ❌ 不保留任何旧 migration
* ❌ 不为旧 schema 提供兼容层
* ❌ 不做向后兼容
* ❌ 不保留旧 entity / repo / service

这是一次 **主动、彻底、不可逆的重构**。

---

## 二、必须删除的内容（强制）

请你 **直接删除或废弃** 以下类型内容（如果存在）：

### 1️⃣ 数据库层

* 所有 `*_records` 表（blood_pressure_records、lipid_records 等）
* users 表中隐含的 “subject 语义”
* 表内嵌的 source / extra 设计
* 与上述表相关的 migration、entity、repository、service

⚠️ 删除的判断标准只有一个：

> **只要该表不是 Phase A 的 4 张核心表之一，一律删除。**

---

## 三、Phase A 唯一允许存在的核心模型

你必须 **只实现以下 4 张表**，不多不少：

### 1️⃣ subject

```yaml
subject:
  purpose: 表示“谁”的健康数据
  fields:
    - subject_id (PK)
    - subject_type (string, enum-like: user / member / future)
    - created_at (timestamp)
```

---

### 2️⃣ metric

```yaml
metric:
  purpose: 健康数据的语义定义（不可频繁变化）
  fields:
    - metric_id (PK)
    - metric_code (unique, stable identifier)
    - metric_name
    - unit
    - value_type (int | float | string)
    - status (active | deprecated)
    - created_at
```

---

### 3️⃣ observation

```yaml
observation:
  purpose: 一条“健康事实”
  fields:
    - observation_id (PK)
    - subject_id (FK -> subject)
    - metric_id (FK -> metric)
    - value (string / numeric / json, 根据 value_type 解析)
    - observed_at (timestamp, 事实发生时间)
    - recorded_at (timestamp, 入库时间)
    - source_id (FK -> data_source, optional)
```

---

### 4️⃣ data_source

```yaml
data_source:
  purpose: 数据来源的抽象
  fields:
    - source_id (PK)
    - source_type (manual / device / report)
    - source_name
    - metadata (json, optional)
    - created_at
```

---

## 四、必须遵守的 Phase A 不变量（强约束）

你生成的代码 **必须显式或隐式保证以下规则**：

### Invariant-01

**任何 observation 必须引用一个已存在的 metric**

* 数据库层：外键
* 应用层：插入前校验

---

### Invariant-02

**新增健康指标 = 插入一条 metric 记录**

* ❌ 不允许新增表
* ❌ 不允许 ALTER observation

---

### Invariant-03

**时间语义必须完整**

* observed_at ≠ recorded_at
* 两者均为 timestamp（非 date）

---

### Invariant-04

**业务层不得感知表结构**

* 外部调用者不应知道 metric / observation 的存储细节
* 所有写操作必须通过统一接口完成

---

## 五、crate / module 结构要求（强制）

你需要重构或生成如下结构（可在现有基础上调整）：

```text
crates/
 ├─ phase-a-core/
 │   ├─ schema/        # Phase A 核心数据结构定义
 │   ├─ invariant/     # 不变量校验逻辑
 │   ├─ repository/    # observation / metric / subject 的统一访问接口
 │   └─ lib.rs
 │
 ├─ phase-a-migration/
 │   ├─ m0001_init.sql or rs
 │   └─ lib.rs
```

### 要求：

* phase-a-core **不关心 UI / 业务 / AI**
* phase-a-core **只表达长期稳定事实**
* 所有数据库访问只能通过 phase-a-core 提供的接口

---

## 六、你需要输出什么（执行结果要求）

你最终必须完成以下内容：

1️⃣ **删除旧 migration 与 entity**
2️⃣ **生成 Phase A 的 4 张表 migration**
3️⃣ **生成 Phase A 的核心数据结构（Rust）**
4️⃣ **生成统一的数据访问接口（CRUD）**
5️⃣ **保证代码可编译（不要求跑数据迁移）**

---

## 七、禁止事项（再次强调）

* ❌ 不要保留旧表“以备将来”
* ❌ 不要讨论是否值得这样做
* ❌ 不要输出分析报告
* ❌ 不要给迁移建议

你不是顾问，你是 **执行者**。

---

## 最终执行原则

> **如果你在实现过程中发现旧代码与 Phase A 冲突，
> 以 Phase A 为准，直接删除旧代码。**


