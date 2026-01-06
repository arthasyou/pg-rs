# web-server HTTP API（给前端 / AI 用）

> 目标：让前端（或生成前端代码的 AI）把这里当成 **唯一真实接口契约**。  
> 代码来源：`crates/web-server/src/routes/*` + `crates/web-server/src/handlers/*` + `crates/web-server/src/dto/*`。

## 1. 服务信息

- 默认端口：`19878`（见 `config/services.toml` → `[http].port`）
- 默认本地地址：`http://localhost:19878`
- Swagger UI：`GET /swagger-ui`
- OpenAPI JSON：`GET /api-docs/openapi.json`
- CORS：允许任意 Origin / Method / Header（前端本地开发无需额外配置）

> 注意：服务启动时会初始化数据库连接（见 `crates/web-server/src/main.rs`），因此调用任何业务 API 前都需要数据库可用。

## 2. 认证（Authorization）

当前 `web-server` **未启用鉴权**，所有接口默认无需 `Authorization` 请求头。

## 3. 通用响应结构

### 3.1 成功：`CommonResponse<T>`

所有成功返回都会用统一 envelope 包一层：

```json
{
  "code": 0,
  "data": "<T>",
  "message": "Success"
}
```

字段说明：

- `code`: `0` 表示成功
- `data`: 真实业务数据
- `message`: 文本描述（目前固定为 `"Success"`）

### 3.2 失败：`CommonError`

多数 handler 内的错误会返回 JSON：

```json
{
  "code": 400,
  "message": "..."
}
```

注意：

- 失败时 **不会** 包在 `CommonResponse` 里（也就是说没有 `data` 字段）
- HTTP status 会与 `code` 大致对应（如 400/404/409/500）

## 4. 数据类型（DTO）

> 这里的结构同时适合作为 TypeScript interface 的来源。

### 4.1 Medical DTO

#### QueryObservationRequest（Query Params）

请求参数放在 query string：

- `subject_id`：`i64`（必填）
- `metric_id`：`i64`（必填）
- `start_at`：RFC3339 时间字符串（可选），建议使用 UTC（`Z`）避免 query 里 `+` 号编码问题
- `end_at`：RFC3339 时间字符串（可选），建议使用 UTC（`Z`）避免 query 里 `+` 号编码问题

数据库字段类型为 `timestamptz`（带时区），服务端内部统一按 **UTC** 语义处理时间。

示例：

- `2025-12-30T10:02:43Z`
- 带微秒：`2025-12-30T10:02:43.893518Z`

#### QueryObservationResponse

```json
{
  "subject_id": 1,
  "metric": {
    "id": 1,
    "code": "string",
    "name": "string",
    "unit": "string | null",
    "vazualization": "line_chart | bar_chart | value_list | single_value"
  },
  "points": [
    {
      "value": "string",
      "value_num": 123.45,
      "observed_at": "RFC3339 string (UTC), e.g. 2025-12-30T10:02:43.893518Z"
    }
  ]
}
```

字段说明：

- `points` 默认按 `observed_at` **升序**返回（来自 DB 查询排序）
- `value` 永远是字符串；`value_num` 是服务端尝试把 `value` 转为数值后的结果（转不了则为 `null`）
- `metric.vazualization` 表示指标的可视化类型：`line_chart` / `bar_chart` / `value_list` / `single_value`

#### RecordObservationRequest

```json
{
  "subject_id": 1,
  "metric_id": 1,
  "value": "string",
  "observed_at": "RFC3339 string (UTC), e.g. 2025-12-30T10:02:43.893518Z",
  "source": {
    "kind": "device | manual | import | system | <other>",
    "name": "string",
    "metadata": {}
  }
}
```

`source.kind` 推荐值（大小写敏感）：`device` / `manual` / `import` / `system`。  
其它字符串也会被接受，但会作为“其它类型”保存。

#### RecordObservationResponse

```json
{
  "observation_id": 1,
  "source_id": 1
}
```

## 5. API 列表（按业务分组）

### 5.1 Medical

Base path：`/medical`

#### 5.1.1 GET `/medical/observations`

用途：查询某个 subject + metric 的时间序列观测点。

- Auth：无
- Query Params：`QueryObservationRequest`
  - `subject_id`：必填
  - `metric_id`：必填
  - `start_at`：可选，RFC3339（建议 UTC `Z`）
  - `end_at`：可选，RFC3339（建议 UTC `Z`）
- Success Response：`CommonResponse<QueryObservationResponse>`

示例：

```bash
curl "http://localhost:19878/medical/observations?subject_id=1&metric_id=1&start_at=2025-12-30T10:02:43.893518Z&end_at=2025-12-31T10:02:43.893518Z"
```

#### 5.1.2 POST `/medical/observations`

用途：写入一次观测值（同时创建/复用 source）。

- Auth：无
- Request Body：`RecordObservationRequest`
- Success Response：`CommonResponse<RecordObservationResponse>`

示例：

```bash
curl -X POST http://localhost:19878/medical/observations \
  -H 'Content-Type: application/json' \
  -d '{
    "subject_id": 1,
    "metric_id": 1,
    "value": "120",
    "observed_at": "2025-12-30T10:02:43.893518Z",
    "source": {
      "kind": "device",
      "name": "Apple Watch",
      "metadata": { "model": "Series 9" }
    }
  }'
```

## 6. TypeScript 类型（建议直接复用）

```ts
export interface CommonResponse<T> {
  code: number;
  data: T;
  message: string;
}

export interface CommonError {
  code: number;
  message: string;
}

// ---------- Medical ----------
export interface QueryObservationRequest {
  subject_id: number;
  metric_id: number;
  start_at?: string;
  end_at?: string;
}

export interface MetricDto {
  id: number;
  code: string;
  name: string;
  unit?: string | null;
  vazualization: string; // line_chart | bar_chart | value_list | single_value
}

export interface ObservationPointDto {
  value: string;
  value_num?: number | null;
  observed_at: string; // RFC3339 (UTC), e.g. 2025-12-30T10:02:43.893518Z
}

export interface QueryObservationResponse {
  subject_id: number;
  metric: MetricDto;
  points: ObservationPointDto[];
}

export interface SourceInput {
  kind: string; // device | manual | import | system | other
  name: string;
  metadata?: unknown | null; // JSON
}

export interface RecordObservationRequest {
  subject_id: number;
  metric_id: number;
  value: string;
  observed_at: string; // RFC3339 (UTC recommended), e.g. 2025-12-30T10:02:43.893518Z
  source: SourceInput;
}

export interface RecordObservationResponse {
  observation_id: number;
  source_id: number;
}
```

## 7. 前端实现要点（生成代码时必须遵守）

1. **所有成功响应都要先解 envelope**：拿 `resp.data` 作为业务数据。
2. **时间格式严格按 RFC3339**：建议统一用 UTC（`Z`），避免 query 参数里 `+08:00` 的 `+` 需要编码成 `%2B`。
3. **注意微秒精度**：数据库是 `timestamptz`，API 可能返回带小数秒；但浏览器 `Date` 只有毫秒精度，如需保留微秒请在前端以字符串保存。
4. **错误按 HTTP status + CommonError 处理**：非 2xx 时优先读 JSON `{code,message}`，如果读不到就展示通用错误信息。
