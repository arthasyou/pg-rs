# pg-rs AI 开发协议

> 本文档是 **执行规约**，AI 必须严格按步骤执行。
> 不允许引入未明确要求的结构、逻辑或抽象。

---

## 一、项目架构与职责边界

```
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  Layer 5: HTTP API
├──────────────────────────────────────────────────────┤
│                     demo-db                          │  Layer 4: Business API
├──────────────────────────────────────────────────────┤
│                    pg-tables                         │  Layer 3: Domain Services
├──────────────────────────────────────────────────────┤
│                     pg-core                          │  Layer 2: Infrastructure
├──────────────────────────────────────────────────────┤
│                    migration                         │  Layer 1: Schema
└──────────────────────────────────────────────────────┘
```

### Layer 1: migration
| 项目 | 说明 |
|------|------|
| 职责 | 数据库表结构定义 |
| 允许 | 新增/修改 migration 文件 |
| 禁止 | 写业务逻辑、写 Rust struct（除 migration 所需） |

### Layer 2: pg-core
| 项目 | 说明 |
|------|------|
| 职责 | 基础能力（连接池、错误、Repository trait、分页） |
| 允许 | 调用 pg-core 提供的 API（如 DbContext、PgError、impl_repository! 等） |
| 禁止 | 修改 pg-core 的代码、接口或行为 |
| 规则 | **永远不动**，新增表/业务不需要修改此层 |

### Layer 3: pg-tables
| 项目 | 说明 |
|------|------|
| 职责 | 表级事实模型 + 单表 Service |
| 包含 | entity（自动生成）、dto、service |
| 规则 | 单表操作，不跨表 JOIN，不校验外键 |

### Layer 4: demo-db
| 项目 | 说明 |
|------|------|
| 职责 | 业务 API 层，编排多个 Service |
| 包含 | api（如 HealthApi）、dto（业务级 DTO） |
| 规则 | 可跨表组合，处理业务语义，不直接操作数据库 |

### Layer 5: web-server
| 项目 | 说明 |
|------|------|
| 职责 | HTTP 接口适配 |
| 包含 | routes、handlers、dto（请求/响应）、error |
| 规则 | 参数校验、错误转换、调用 demo-db API |

---

## 二、AI 能做与不能做

### 允许 AI 执行的任务

| 任务类型 | 涉及 Crate | 说明 |
|----------|-----------|------|
| 新增表 | migration, pg-tables | 按协议流程执行 |
| 新增/修改 Service | pg-tables | 单表 CRUD 操作 |
| 新增/修改业务 API | demo-db | 编排多个 Service |
| 新增/修改 HTTP 接口 | web-server | 路由、Handler、DTO |
| 修复 Bug | 相关层 | 不改变架构 |

### 禁止 AI 执行的任务

| 禁止行为 | 原因 |
|----------|------|
| 修改 pg-core | 基础设施层稳定，不随业务变化 |
| 在 pg-tables 中跨表 JOIN | 违反单表原则 |
| 在 pg-tables 中校验外键存在 | 这是 demo-db 的职责 |
| 在 Service 中写业务规则 | 这是 demo-db 的职责 |
| 在 DTO 中写逻辑 | DTO 只是数据容器 |
| 手写 entity | entity 由 sea-orm-cli 生成 |
| 新增未要求的抽象 | 不发明，只执行 |

---

## 三、新增表标准流程

### Step 1: migration（Layer 1）

```bash
# 生成新 migration
sea-orm-cli migrate generate <table_name> -d crates/migration
```

编辑生成的文件，定义表结构：

```rust
// crates/migration/src/m0002_xxx.rs
manager.create_table(
    Table::create()
        .table(TableName::Table)
        .col(ColumnDef::new(Column::Id).big_integer().auto_increment().primary_key())
        .col(ColumnDef::new(Column::Name).string().not_null())
        .col(ColumnDef::new(Column::CreatedAt).timestamp().not_null())
        .to_owned(),
)
```

### Step 2: 生成 Entity

```bash
# 运行脚本（会重建数据库）
./scripts/fresh_db.sh
```

结果：`crates/pg-tables/src/entity/<table_name>.rs` 自动生成

> **禁止手写 entity，脚本会覆盖**

### Step 3: pg-tables DTO（Layer 3）

位置：`crates/pg-tables/src/table/<table_name>/dto.rs`

```rust
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 事实结构体（完整数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xxx {
    pub id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
}

/// 创建参数（不含 id、created_at）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateXxx {
    pub name: String,
}

/// 查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct QueryXxx {
    pub name: Option<String>,
}
```

DTO 规则：
- 只表示输入/输出数据
- 不生成 id / created_at（由 Service 处理）
- 不包含业务逻辑
- DTO 中的字段必须来源于 migration 定义 / entity 生成的字段，禁止凭空新增字段名

### Step 4: pg-tables Service（Layer 3）

位置：`crates/pg-tables/src/table/<table_name>/service.rs`

```rust
use sea_orm::*;
use pg_core::{DbContext, PgError, Result, impl_repository};
use crate::entity::xxx;
use super::dto::{Xxx, CreateXxx};

impl_repository!(XxxRepo, xxx::Entity, xxx::Model);

pub struct XxxService {
    db: DbContext,
}

impl XxxService {
    pub fn new(db: DbContext) -> Self {
        Self { db }
    }

    pub async fn create(&self, input: CreateXxx) -> Result<Xxx> {
        let model = xxx::ActiveModel {
            name: Set(input.name),
            created_at: Set(time::OffsetDateTime::now_utc()),
            ..Default::default()
        };
        let result = XxxRepo::insert(&self.db, model).await?;
        Ok(Self::from_model(result))
    }

    pub async fn get(&self, id: i64) -> Result<Xxx> {
        let model = XxxRepo::find_by_id(&self.db, id)
            .await?
            .ok_or_else(|| PgError::not_found("Xxx", id))?;
        Ok(Self::from_model(model))
    }

    fn from_model(model: xxx::Model) -> Xxx {
        Xxx {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
        }
    }
}
```

Service 规则：
- 单表操作，不 JOIN
- 不校验外键是否存在
- id 由数据库生成
- created_at 由 Service 生成
- 使用 Repository 宏
- Service 中使用的字段必须与 entity 中的字段一致，禁止假设不存在的字段

### Step 5: pg-tables mod.rs 导出

```rust
// crates/pg-tables/src/table/<table_name>/mod.rs
pub mod dto;
pub mod service;

pub use dto::*;
pub use service::*;
```

```rust
// crates/pg-tables/src/table/mod.rs
pub mod <table_name>;
```

---

## 四、新增业务 API 流程

### Step 1: demo-db DTO（Layer 4）

位置：`crates/demo-db/src/dto/<domain>.rs`

```rust
use serde::{Deserialize, Serialize};

/// 业务请求参数（可组合多表查询条件）
#[derive(Debug, Clone, Deserialize)]
pub struct BusinessRequest {
    pub subject_id: i64,
    pub metric_id: i64,
}

/// 业务响应（可聚合多表数据）
#[derive(Debug, Clone, Serialize)]
pub struct BusinessResponse {
    pub subject: SubjectInfo,
    pub metrics: Vec<MetricData>,
}
```

### Step 2: demo-db API（Layer 4）

位置：`crates/demo-db/src/api/<domain>.rs`

```rust
use pg_core::DbContext;
use pg_tables::table::{subject::SubjectService, metric::MetricService};
use crate::dto::{BusinessRequest, BusinessResponse};

pub struct BusinessApi {
    subject_svc: SubjectService,
    metric_svc: MetricService,
}

impl BusinessApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            subject_svc: SubjectService::new(db.clone()),
            metric_svc: MetricService::new(db),
        }
    }

    pub async fn query(&self, req: BusinessRequest) -> Result<BusinessResponse> {
        // 编排多个 Service
        let subject = self.subject_svc.get(req.subject_id).await?;
        let metrics = self.metric_svc.list_by_subject(req.subject_id).await?;

        // 组合响应
        Ok(BusinessResponse { subject, metrics })
    }
}
```

API 规则：
- 可跨表组合数据
- 可包含业务校验
- 通过 Service 操作，不直接访问数据库

---

## 五、新增 HTTP 接口流程

### Step 1: web-server DTO（Layer 5）

位置：`crates/web-server/src/dto/<domain>.rs`

```rust
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Debug, Deserialize, IntoParams)]
pub struct QueryRequest {
    pub subject_id: i64,
    pub metric_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryResponse {
    pub data: Vec<DataItem>,
}
```

### Step 2: web-server Handler（Layer 5）

位置：`crates/web-server/src/handlers/<domain>.rs`

```rust
use axum::extract::Query;
use demo_db::api::BusinessApi;
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};
use crate::{dto::QueryRequest, error::Error, statics::db_manager::get_default_ctx};

#[utoipa::path(
    get,
    path = "/endpoint",
    tag = "Domain",
    params(QueryRequest),
    responses(
        (status = 200, body = CommonResponse<QueryResponse>),
    )
)]
pub async fn query_handler(
    Query(req): Query<QueryRequest>,
) -> ResponseResult<QueryResponse> {
    let api = BusinessApi::new(get_default_ctx());

    let result = api.query(req.into())
        .await
        .map_err(|e| Error::Custom(e.to_string()))?;

    Ok(result.into_common_response().to_json())
}
```

Handler 规则：
- 禁止将所有错误统一转换为字符串（如 `.to_string()`）
- 必须保留基本错误类别（NotFound / Validation / Database / Unauthorized）
- 错误类别应映射到对应的 HTTP 状态码

### Step 3: web-server Route（Layer 5）

位置：`crates/web-server/src/routes/<domain>.rs`

```rust
use axum::routing::get;
use utoipa_axum::router::OpenApiRouter;
use crate::handlers::domain::query_handler;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/endpoint", get(query_handler))
}
```

注册到主路由：`crates/web-server/src/routes/mod.rs`

---

## 六、自检清单

### 新增表时
- [ ] 只修改了 migration + pg-tables
- [ ] pg-core 完全未动
- [ ] Service 只操作单表
- [ ] DTO 只是数据容器
- [ ] Entity 由脚本生成，未手写
- [ ] DTO / Service 字段均来源于 entity，未凭空新增

### 新增业务 API 时
- [ ] 放在 demo-db 层
- [ ] 通过 Service 操作，不直接查数据库
- [ ] 业务逻辑在 API 层，不在 Service 层

### 新增 HTTP 接口时
- [ ] 放在 web-server 层
- [ ] 调用 demo-db API，不直接用 Service
- [ ] 错误正确转换为 HTTP 响应
- [ ] 错误保留类别，未统一字符串化
- [ ] OpenAPI 注解完整

---

## 七、设计哲学

> **事实优先（fact-first）建模**
> - 表 = 已发生的事实
> - Service = 事实的写入/读取（单表）
> - API = 业务语义（跨表编排）
> - Handler = HTTP 适配
>
> **AI 的职责是展开事实，不是发明规则。**
