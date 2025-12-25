# 如何在本项目中新增一张表（AI 执行规范）

> 本文档是 **执行规约**，不是设计讨论。  
> AI 必须严格按步骤执行，不允许引入未明确要求的结构、逻辑或抽象。

---

## 一、项目结构与职责边界（必须遵守）

本项目包含 **3 个 crate**：

### 1️⃣ migration
- **职责**：数据库表结构定义
- **唯一允许的操作**：
  - 新增 migration
  - 修改数据库 schema
- **禁止事项**：
  - ❌ 不写业务逻辑
  - ❌ 不写 Rust struct（除 migration 所需）

### 2️⃣ pg-core
- **职责**：基础能力（分页、错误、repo 抽象等）
- **规则**：
  - ❌ 新增表 **不需要** 修改 pg-core
  - ❌ 不新增任何表相关代码
- **结论**：**pg-core 永远不动**

### 3️⃣ pg-tables
- **职责**：表级事实模型 + 单表 service
- **所有新表的逻辑代码都在这里完成**

---

## 二、新增一张表的标准执行流程（严格顺序）

### Step 1：在 migration 中新增表
1. 新建 migration 文件
2. 定义表结构（字段 + 类型 + 索引）
3. 只做 schema 层面的事情，不引入任何业务语义

> migration 的目标是：**数据库能建表**

### Step 2：用脚本生成 entity（必须用 `scripts/fresh_db.sh`）
1. 运行 `scripts/fresh_db.sh`
2. 脚本行为：
   - `migrate refresh`（**会重建数据库**）
   - `generate entity` 输出到 `crates/pg-tables/src/entity`
   - 生成包含 `entity::<table_name>`、`Model`、`ActiveModel`、`Column`
3. 确保本地有 `sea-orm-cli` 且数据库环境可用

> ⚠️ **禁止手写 entity；脚本会直接覆盖生成结果**

### Step 3：确认 entity 结果（不做手工搬运）
1. `crates/pg-tables/src/entity` 下应出现新表 entity 文件
2. `mod.rs` / `prelude.rs` 由生成器维护，无需手改
3. entity 中不写任何业务代码，只保留生成内容

---

## 三、在 pg-tables 中需要 AI 自动完成的工作

> 从这一步开始，AI 负责 **展开逻辑**；人类只负责审查是否越界。

### Step 4：为新表创建 dto 模块

位置示例：
```
pg-tables/src/table/<table_name>/dto.rs
```

#### 必须包含以下内容（如适用）：
1. **事实结构体**
```rust
pub struct Xxx { ... }
```
2. **强类型 ID**
```rust
pub struct XxxId(pub i64);
```
3. **Create DTO（写入参数）**
```rust
pub struct CreateXxx { ... }
```
4. **List / Query DTO（查询参数）**
```rust
pub struct ListXxx { ... }
```

#### DTO 规则（强制）：
- DTO **只表示输入参数**
- DTO 不生成 id / created_at
- DTO 不做跨表校验
- DTO 不包含业务逻辑

### Step 5：为新表创建 service 模块

位置示例：
```
pg-tables/src/table/<table_name>/service.rs
```

#### Service 必须遵守的规则：
1. **单表原则**
   - ❌ 不 JOIN
   - ❌ 不访问其它表
   - ❌ 不校验外键是否存在
2. **create / record 行为**
   - id 由数据库生成
   - created_at / recorded_at 由 service 生成
   - 返回完整事实结构体
3. **get / exists**
   - 仅限按 id 或表内唯一键
   - 不引入业务判断
4. **list**
   - 使用 `PaginationInput`
   - 排序字段固定（created_at / observed_at）
   - 可选过滤，不做默认业务偏好

### Step 6：Repository 使用规范
- 使用：
```rust
impl_repository!(XxxRepo, XxxEntity, xxx::Model);
```
- 所有数据库操作通过 repo
- 不直接写 SQL
- 不在 service 中拼接业务查询

### Step 7：Model → Struct 映射（必须）

每个 service **必须有**：
```rust
fn from_model(model: xxx::Model) -> Xxx
```

规则：
- ❌ 不补默认值
- ❌ 不做推理
- ❌ 不解释字段含义
- ✅ 只是字段映射

---

## 四、严格禁止的行为（AI 必须避免）

以下行为一律视为 **错误实现**：
- ❌ 在 pg-core 中新增表相关代码
- ❌ 在 service 中校验其它表是否存在
- ❌ 在 DTO 中写业务规则
- ❌ 在 entity / model 中写逻辑
- ❌ 在单表 service 中做跨表 JOIN
- ❌ 在 create / record 中引入隐含业务判断

---

## 五、判断是否“写对了”的自检清单（AI 自查）

- [ ] 新表是否只改了 migration + pg-tables
- [ ] pg-core 是否完全未改动
- [ ] service 是否只操作单表
- [ ] DTO 是否只用于输入
- [ ] 是否可以用现有表作为模板直接对照
- [ ] 是否没有新增任何“聪明但未要求的逻辑”

---

## 六、设计哲学（只读，不可违反）

> 本项目采用 **事实优先（fact-first）** 的建模方式：  
> - 表 = 已发生的事实  
> - service = 事实的写入 / 读取  
> - business = 跨表语义（不在 pg-tables 中）  
>  
> **AI 的职责是展开事实，不是发明规则。**
