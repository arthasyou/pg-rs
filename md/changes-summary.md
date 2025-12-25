# pg-sdk 表层改动详细总结

> 目的：让评估方可以直接理解“做了什么、为什么做、影响到哪些 API 与数据结构”。

## 1. 变更动机与总体原则
- **Phase A 文档对齐**：四表模型（subject/metric/observation/data_source）必须完整、稳定、只表达事实。
- **DTO 语义清晰化**：所有对外写入改为“输入参数 DTO”，避免函数参数膨胀，同时为未来扩展字段保留向后兼容空间。
- **服务边界明确**：service 层维持**单表语义**，避免跨表校验或组合逻辑。
- **时间类型统一**：DTO 时间字段统一使用 `PrimitiveDateTime`。
- **列表统一分页**：所有 list 接口统一使用 `page + limit` 的分页策略，分页参数独立可选并有默认值。
- **事实只由存储层确认**：Observation 不允许在 core 层自由构造，避免未持久化事实。

---

## 2. DTO 结构详细变化

### 2.1 通用 DTO（table/dto.rs）
- **新增** `PaginationInput`，统一 list 分页入参。
  ```rust
  pub struct PaginationInput {
      pub page: u64,
      pub limit: u64,
  }
  ```
- 提供 `to_params()` 转换为 `pg-core::PaginationParams` 并进行 `validate()`。
- `Default`：`page=1`、`limit=20`。

### 2.2 subject
- **新增** `CreateSubject`：用于封装创建参数。
  ```rust
  pub struct CreateSubject {
      pub kind: SubjectKind,
  }
  ```
- **新增** `ListSubject`：用于列表过滤参数。
  ```rust
  pub struct ListSubject {
      pub kind: Option<SubjectKind>,
  }
  ```
- `Subject` 维持三要素：`id` / `kind` / `created_at`（时间为 `PrimitiveDateTime`）。

### 2.3 metric
- **新增** `CreateMetric`：用于封装创建参数。
  ```rust
  pub struct CreateMetric {
      pub code: MetricCode,
      pub name: String,
      pub unit: Option<String>,
      pub value_type: MetricValueType,
  }
  ```
- **新增** `ListMetric`：用于列表过滤参数。
  ```rust
  pub struct ListMetric {
      pub status: Option<MetricStatus>,
      pub value_type: Option<MetricValueType>,
  }
  ```
- **补齐枚举字符串映射**：
  - `MetricValueType`：支持 `int/float/decimal/bool/text/string`，未知值默认走 `Text`。
  - `MetricStatus`：支持 `active/deprecated`，未知值默认 `Active`。

### 2.4 observation
- **新增** `RecordObservation`：用于封装 record 参数。
  ```rust
  pub struct RecordObservation {
      pub subject_id: SubjectId,
      pub metric_id: MetricId,
      pub value: ObservationValue,
      pub observed_at: PrimitiveDateTime,
      pub source_id: Option<DataSourceId>,
  }
  ```
- **新增**列表过滤 DTO：
  ```rust
  pub struct ListObservationBySubject { pub subject_id: SubjectId }
  pub struct ListObservationByMetric { pub metric_id: MetricId }
  pub struct ListObservationByTimeRange {
      pub start: PrimitiveDateTime,
      pub end: PrimitiveDateTime,
      pub subject_id: Option<SubjectId>,
      pub metric_id: Option<MetricId>,
  }
  ```
- **补齐字段**：`source_id: Option<DataSourceId>`。
- **移除** `Observation::new`：禁止在 core 层构造“未持久化事实”。

### 2.5 data_source
- **补齐字段**：`metadata: Option<JsonValue>`、`created_at: PrimitiveDateTime`。
- **新增** `CreateDataSource`：用于封装创建参数。
  ```rust
  pub struct CreateDataSource {
      pub kind: DataSourceKind,
      pub name: String,
      pub metadata: Option<JsonValue>,
  }
  ```
- **新增** `ListDataSource`：用于列表过滤参数。
  ```rust
  pub struct ListDataSource {
      pub kind: Option<DataSourceKind>,
  }
  ```
- **完善枚举** `DataSourceKind`：固定类型 + `Other(String)`。

---

## 3. Service 接口与行为变化

### 3.1 subject
- `SubjectService::create` 签名变更：
  - **旧**：`create(kind: SubjectKind)`
  - **新**：`create(input: CreateSubject)`
- **新增** `list`：
  - 签名：`list(input: ListSubject, pagination: Option<PaginationInput>)`
  - 按 `SubjectKind` 可选过滤
  - 排序：`created_at` 降序
  - 返回：`PaginatedResponse<Subject>`

### 3.2 metric
- `MetricService::create` 签名变更：
  - **旧**：`create(code, name, unit, value_type)`
  - **新**：`create(input: CreateMetric)`
- `create` 仍会检查 `metric_code` 是否重复（存在即返回 `already_exists`）。
- 保留并明确以下能力：
  - `get` / `get_by_code`
  - `exists` / `exists_by_code`
  - `deprecate`（受控更新：将 status 置为 deprecated）
- **新增** `list`：
  - 签名：`list(input: ListMetric, pagination: Option<PaginationInput>)`
  - 按 `status` / `value_type` 可选过滤
  - 排序：`created_at` 降序
  - 返回：`PaginatedResponse<Metric>`

### 3.3 observation
- `ObservationService::record` 签名变更：
  - **旧**：`record(subject_id, metric_id, value, observed_at, source_id)`
  - **新**：`record(input: RecordObservation)`
- **单表原则**：不做跨表存在性校验（subject/metric/source 均不检查）。
- **list 接口统一分页**：
  - `list_by_subject(input, pagination)`
  - `list_by_metric(input, pagination)`
  - `list_by_time_range(input, pagination)`
  - `pagination` 为 `Option<PaginationInput>`（缺省使用默认值）
  - 排序：`observed_at` 降序
  - 返回：`PaginatedResponse<Observation>`

### 3.4 data_source
- `DataSourceService::create` 签名变更：
  - **旧**：`create(kind, name, metadata)`
  - **新**：`create(input: CreateDataSource)`
- 保留 `get` / `exists`。
- **新增** `list`：
  - 签名：`list(input: ListDataSource, pagination: Option<PaginationInput>)`
  - 按 `kind` 可选过滤
  - 排序：`created_at` 降序
  - 返回：`PaginatedResponse<DataSource>`

---

## 4. 分页策略与默认值
- 使用 `pg-core::PaginationParams` + `validate()`：
  - `page=0` → 自动归一为 `1`
  - `limit=0` → 自动归一为 `20`
  - `limit>100` → 自动裁剪为 `100`
- 所有 list API 统一通过 `repo.find_paginated(...)` 执行。
- `PaginationInput` 为可选参数，`None` 时使用默认值。

---

## 5. 数据结构与实体映射对齐
- `data_source` DTO 与实体结构对齐：`metadata` / `created_at`。
- `observation` DTO 增加 `source_id` 与实体字段一致。
- `metric` / `subject` 时间字段类型对齐。

---

## 6. 依赖与配置变更
- 新增 `serde_json` 作为 `data_source.metadata` 的承载类型。
  - `Cargo.toml`（workspace）新增 `serde_json = "1.0"`
  - `crates/pg-sdk/Cargo.toml` 引入 `serde_json.workspace = true`

---

## 7. 代码层面影响清单

### 新增 DTO
- `PaginationInput`（通用）
- `CreateSubject` / `ListSubject`
- `CreateMetric` / `ListMetric`
- `RecordObservation`
- `ListObservationBySubject` / `ListObservationByMetric` / `ListObservationByTimeRange`
- `CreateDataSource` / `ListDataSource`

### Service 接口签名变更
- `SubjectService::create(CreateSubject)`
- `MetricService::create(CreateMetric)`
- `ObservationService::record(RecordObservation)`
- `DataSourceService::create(CreateDataSource)`
- 新增 `list` / `list_by_*` 分页接口（均接收 `Option<PaginationInput>`）

### 行为调整
- `ObservationService` 移除跨表存在性校验，保持纯单表操作。
- `Observation::new` 被移除，禁止在 core 层构造“未持久化事实”。
- list 查询统一分页与排序规范。

---

## 8. 涉及文件清单
- `crates/pg-sdk/src/table/dto.rs`
- `crates/pg-sdk/src/table/subject/dto.rs`
- `crates/pg-sdk/src/table/subject/service.rs`
- `crates/pg-sdk/src/table/metric/dto.rs`
- `crates/pg-sdk/src/table/metric/service.rs`
- `crates/pg-sdk/src/table/observation/dto.rs`
- `crates/pg-sdk/src/table/observation/service.rs`
- `crates/pg-sdk/src/table/data_source/dto.rs`
- `crates/pg-sdk/src/table/data_source/service.rs`
- `Cargo.toml`
- `crates/pg-sdk/Cargo.toml`

---

## 9. 可选后续动作（未执行）
- 为输入 DTO 增加轻量校验逻辑（仅字段合法性，不跨表）。
- 视需要扩展 list 过滤字段（如 `metric_code` 前缀、`source_name` 模糊查询等）。
