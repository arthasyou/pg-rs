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
),

-- ======================================================
-- 身高（一次即可）
-- ======================================================
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.height'),
    '172',
    '2025-01-10 09:10:00'
),

-- ======================================================
-- 血压（收缩压，5 次）
-- ======================================================

-- 第 1 次
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '128',
    '2025-01-10 09:20:00'
),

-- 第 2 次
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '132',
    '2025-02-15 08:50:00'
),

-- 第 3 次
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '125',
    '2025-03-18 09:05:00'
),

-- 第 4 次
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '130',
    '2025-04-22 08:40:00'
),

-- 第 5 次
(
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '127',
    '2025-05-30 09:00:00'
);