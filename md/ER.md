``` mermaid
erDiagram
    SUBJECT {
        bigint subject_id PK
        string subject_type
        datetime created_at
    }

    METRIC {
        bigint metric_id PK
        string metric_code
        string metric_name
        string unit
        string value_type
        string status
        datetime created_at
    }

    OBSERVATION {
        bigint observation_id PK
        bigint subject_id FK
        bigint metric_id FK
        string value
        datetime observed_at
        datetime recorded_at
        bigint source_id FK
    }

    DATA_SOURCE {
        bigint source_id PK
        string source_type
        string source_name
        json metadata
    }

    SUBJECT ||--o{ OBSERVATION : has
    METRIC  ||--o{ OBSERVATION : defines
    DATA_SOURCE ||--o{ OBSERVATION : provides
```