# Web Server HTTP API (For Frontend + AI)

This document describes the **current** HTTP endpoints exposed by `crates/web-server`.
All medical endpoints are under `/medical`. LLM endpoints are under `/llm`.
Swagger UI is available at `/swagger-ui`, and OpenAPI JSON at `/api-docs/openapi.json`.

## Conventions

### CommonResponse envelope (medical endpoints)
All successful `/medical/*` responses are wrapped in:

```json
{
  "code": 0,
  "message": "ok",
  "data": { /* payload */ }
}
```

Error responses are **not** wrapped and look like:

```json
{
  "code": 400,
  "message": "error message"
}
```

### Time format
Use **RFC3339** timestamps (UTC recommended). Example: `2025-12-30T10:02:43.893518Z`.

### Metric vs Recipe
All metrics are unified in the **metric** table. Composite metrics are configured in **recipe**.
Querying observations always uses `metric_id`. If the metric is derived, the server will look up
its recipe and evaluate it. Frontend does not query recipes directly.

---

## /medical endpoints

### 1) GET /medical/observations
Query observations for a metric (primitive or derived). This is the **only** query endpoint
(frontend should not call `/medical/recipes/observations`; it no longer exists).

**Query params (QueryObservationParams):**
- `subject_id` (i64, required)
- `metric_id` (i64, required)
- `start_at` (RFC3339, optional; default: unix epoch)
- `end_at` (RFC3339, optional; default: now)

**Response:** `CommonResponse<QueryRecipeObservationResponse>`

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "subject_id": 1,
    "metric": {
      "id": 16,
      "metric_code": "blood_glucose",
      "metric_name": "Blood Glucose",
      "unit": "mmol/L",
      "value_type": "float",
      "visualization": "line_chart"
    },
    "points": [
      {
        "value": "5.6",
        "value_num": 5.6,
        "observed_at": "2025-12-30T10:02:43.893518Z"
      }
    ]
  }
}
```

### 2) POST /medical/observations
Record a single observation (primitive metrics only, but server does not strictly enforce this).

**Body (RecordObservationRequest):**
```json
{
  "subject_id": 1,
  "metric_id": 16,
  "value": "5.6",
  "observed_at": "2025-12-30T10:02:43.893518Z",
  "source": {
    "kind": "device",
    "name": "glucometer",
    "metadata": {"serial": "X123"}
  }
}
```

**Response:** `CommonResponse<RecordObservationResponse>`
```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "observation_id": 123,
    "source_id": 45
  }
}
```

### 3) GET /medical/metrics/selectable
Get dropdown options for metrics.

**Response:** `CommonResponse<ListSelectableMetricsResponse>`
```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "metrics": [
      {"id": 16, "name": "Blood Glucose", "unit": "mmol/L"}
    ]
  }
}
```

### 4) POST /medical/data-source/markdown
Upload Markdown content, parse it into JSON, and create a data source.

**Body (UploadMarkdownRequest):**
```json
{
  "source_type": "import",
  "source_name": "lab_report_2025-12",
  "file_content": "# Report..."
}
```

**Response:** `CommonResponse<UploadMarkdownResponse>`
```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "source_id": 45,
    "source_type": "import",
    "source_name": "lab_report_2025-12",
    "parsed_data": {"any": "json"},
    "created_at": "2025-12-30T10:02:43.893518Z"
  }
}
```

### 5) POST /medical/extract-metrics
Use LLM to extract health metrics from Markdown and optionally write observations.

**Body (ExtractHealthMetricsRequest):**
```json
{
  "content": "# Report...",
  "subject_id": 1,
  "source_type": "import",
  "source_name": "lab_report_2025-12"
}
```

**Response:** `CommonResponse<ExtractHealthMetricsResponse>`
```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "data": {
      "patient_info": {"name": "Alice"},
      "metrics": [
        {"metric_code": "blood_glucose", "value": "5.6 mmol/L"}
      ],
      "diagnoses": ["Normal"],
      "recommendations": ["Keep healthy diet"]
    },
    "source_id": 45,
    "records_inserted": 3
  }
}
```

---

## /llm endpoints

### 1) GET /llm/model/info
Returns configured model info (no CommonResponse envelope).

**Response:**
```json
{
  "base_url": "http://localhost:11434",
  "model": "llama3",
  "has_api_key": false
}
```

### 2) POST /llm/generate
Generate text using the configured LLM (example implementation).

**Body:**
```json
{
  "prompt": "Hello",
  "temperature": 0.7,
  "top_p": 0.9
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Would call http://localhost:11434 with model llama3",
  "prompt": "Hello",
  "base_url": "http://localhost:11434",
  "model": "llama3"
}
```

---

## Notes for Frontend AI
- **Always call `/medical/observations`** for both primitive and derived metrics.
- `metric_id` is the primary identifier. Derived metrics are resolved server-side via recipe.
- Treat `value` as string and only use `value_num` when it is present.
- If you need schema guarantees, prefer the OpenAPI JSON: `/api-docs/openapi.json`.
