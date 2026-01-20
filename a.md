请严格遵循《pg-rs AI 开发协议》完成新增表 **`recipe`** 的全部代码改动，仅允许修改/新增以下 crate：`crates/migration`、`crates/pg-tables`、`crates/demo-db`。禁止改动 `pg-core`、禁止在 `pg-tables` 跨表 JOIN、禁止在 `pg-tables` 校验外键存在、禁止手写 entity（entity 由 `./scripts/fresh_db.sh` 生成，你只需假设生成后会存在 `crates/pg-tables/src/entity/recipe.rs` 并按它来写 service/DTO 的字段映射）。

## 目标：新增 recipe 表（综合指标定义/配置表）

### 业务语义

- `recipe` 用来描述一个“综合指标”的计算配置（计算逻辑在程序里实现）。
- DB 存：依赖哪些指标（list）、以及如何调用程序里的算法（函数名/版本 key + 参数映射/可选可读表达式）。
- 计算结果仍落在现有 `observation`（不在本任务内实现计算器或回写逻辑）。

---

## Step 1：migration（Layer 1）

1. 生成 migration：
   `sea-orm-cli migrate generate recipe -d crates/migration`

2. 在生成的 migration 文件中创建表 `recipe`，字段只要满足最关键需求并遵循项目已有风格（请参考现有表是否有 `created_at/updated_at` 等约定并保持一致）。

**必须字段（关键字段）**

- `id`：big_integer auto_increment primary key
- `output_metric_id`：big_integer NOT NULL
  - 说明：产出哪个综合指标（对应现有的 metric 字典表的 id；此处不做外键校验逻辑）

- `deps`：jsonb NOT NULL
  - 说明：依赖的 metric_id 列表（list）

- `calc_key`：string/text NOT NULL
  - 说明：程序内算法实现的 key，例如 `fib4_v1`

- `arg_map`：jsonb NULL
  - 说明：参数映射（例如 `{ "AST": 123, "ALT": 456 }`，值为 metric_id）

- `expr`：jsonb NULL
  - 说明：可读表达式/说明（不用于 DB 执行）

- 以及项目惯例的时间字段（如 `created_at`、`updated_at`）：timestamp NOT NULL（若项目其他表有则必须保持一致）

> 注意：migration 只定义表结构，不写业务逻辑。

---

## Step 2：生成 Entity（说明而非执行）

按协议后续需要运行 `./scripts/fresh_db.sh` 来生成 entity；你不要手写 entity。你在后续代码里直接假设 entity 存在并字段名与 migration 一致。

---

## Step 3：pg-tables DTO（Layer 3）

新增目录与文件：

- `crates/pg-tables/src/table/recipe/dto.rs`
- `crates/pg-tables/src/table/recipe/service.rs`
- `crates/pg-tables/src/table/recipe/mod.rs`
  并在：
- `crates/pg-tables/src/table/mod.rs` 增加 `pub mod recipe;`

### dto.rs 要求

使用 `serde::{Serialize, Deserialize}`，时间用 `time::OffsetDateTime`（禁止 chrono）。

定义至少以下 DTO（字段必须来源于 entity / migration）：

- `Recipe`（完整输出）
  - `id: i64`
  - `output_metric_id: i64`
  - `deps: serde_json::Value`（对应 jsonb）
  - `calc_key: String`
  - `arg_map: Option<serde_json::Value>`
  - `expr: Option<serde_json::Value>`
  - `created_at: OffsetDateTime`（如果表里有）
  - `updated_at: OffsetDateTime`（如果表里有）

- `CreateRecipe`（创建输入，不含 id 和 created_at/updated_at）
  - `output_metric_id: i64`
  - `deps: serde_json::Value`
  - `calc_key: String`
  - `arg_map: Option<serde_json::Value>`
  - `expr: Option<serde_json::Value>`

- `QueryRecipe`（查询条件，可选）
  - `output_metric_id: Option<i64>`
  - `calc_key: Option<String>`

---

## Step 4：pg-tables Service（Layer 3）

在 `service.rs` 中：

- 使用 `impl_repository!(RecipeRepo, recipe::Entity, recipe::Model);`
- `RecipeService` 只做单表 CRUD，不做 JOIN、不校验 output_metric_id 是否存在。
- `create()`：由 service 写入 `created_at/updated_at = time::OffsetDateTime::now_utc()`（如果表里有这些字段）。
- `get(id)`：找不到用 `PgError::not_found("Recipe", id)`。
- 可加 `list(QueryRecipe)`：只做本表过滤（如按 output_metric_id / calc_key），不跨表。

字段映射必须严格来自 entity 字段名；jsonb 字段用 `sea_orm::Set` + `serde_json::Value`。

---

## Step 5：pg-tables 导出

按协议补齐：

- `crates/pg-tables/src/table/recipe/mod.rs` 导出 dto/service
- `crates/pg-tables/src/table/mod.rs` 注册 recipe module

---

## Step 6：demo-db（Layer 4）

仅做“业务 API 层”封装，不写 web-server。

新增：

- `crates/demo-db/src/dto/recipe.rs`
- `crates/demo-db/src/api/recipe.rs`
  并在相应 `mod.rs` 里注册导出（按项目现有结构照抄风格）。

### demo-db dto/recipe.rs

定义业务层请求/响应 DTO（可直接复用 pg-tables 的 DTO 结构，但不要把逻辑写进 DTO）：

- `CreateRecipeRequest`（同 CreateRecipe）
- `RecipeResponse`（同 Recipe）

### demo-db api/recipe.rs

实现 `RecipeApi`：

- 构造函数 `new(db: DbContext)`，内部持有 `RecipeService`
- 方法：
  - `create(req: CreateRecipeRequest) -> Result<RecipeResponse>`
  - `get(id: i64) -> Result<RecipeResponse>`
  - （可选）`list(req: QueryRecipe) -> Result<Vec<RecipeResponse>>`
    只允许编排 service，不直接操作数据库。

---

## 自检

- [ ] 只改动 migration + pg-tables + demo-db
- [ ] pg-core 未动
- [ ] pg-tables 不跨表 JOIN、不校验外键存在
- [ ] 未手写 entity
- [ ] DTO 字段全部来自 migration/entity
- [ ] 时间使用 `time` crate（禁止 chrono）

输出要求：请给出你新增/修改的所有文件路径与完整代码内容（每个文件一个代码块），确保能编译通过（假设 entity 已由脚本生成）。
