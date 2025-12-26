SELECT
    o.observation_id,
    o.subject_id,
    o.metric_id,
    o.value,
    o.observed_at,
    o.source_id,

    m.metric_id      AS metric_id,
    m.metric_code    AS metric_code,
    m.name           AS metric_name,
    m.unit           AS metric_unit,
    m.value_type     AS metric_value_type,
    m.status         AS metric_status,
    m.created_at     AS metric_created_at
FROM observation o
INNER JOIN metric m
    ON o.metric_id = m.metric_id
WHERE
    o.subject_id = :subject_id
    AND (:metric_id IS NULL OR o.metric_id = :metric_id)
    AND (:from_time IS NULL OR o.observed_at >= :from_time)
    AND (:to_time   IS NULL OR o.observed_at <= :to_time)
ORDER BY
    o.observed_at DESC;
