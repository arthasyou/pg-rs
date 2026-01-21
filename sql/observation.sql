-- ===============================
-- Observation 初始化（体检样本）
-- subject_id = 1
-- 3 次检查，每次 2 个指标（TG + FPG）
-- ===============================

INSERT INTO observation (
    subject_id,
    metric_id,
    value,
    observed_at
)
VALUES

-- ===== 第 1 次检查 =====
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.tg'),
    '2.31',
    '2025-01-10 09:20:00'
),
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.fpg'),
    '5.02',
    '2025-01-10 09:20:00'
),

-- ===== 第 2 次检查 =====
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.tg'),
    '2.08',
    '2025-06-15 09:45:00'
),
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.fpg'),
    '4.96',
    '2025-06-15 09:45:00'
),

-- ===== 第 3 次检查 =====
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.tg'),
    '1.92',
    '2025-12-14 10:30:00'
),
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.fpg'),
    '5.07',
    '2025-12-14 10:30:00'
);
