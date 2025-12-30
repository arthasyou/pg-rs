-- ===============================
-- Observation 初始化（体检样本）
-- subject_id = 1
-- ===============================

INSERT INTO observation (
    observation_id,
    subject_id,
    metric_id,
    value,
    observed_at
)
VALUES
-- 身高
(
    1,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.height'),
    '175',
    '2025-12-14 09:00:00'
),

-- 体重
(
    2,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.weight'),
    '63',
    '2025-12-14 09:00:00'
),

-- BMI
(
    3,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bmi'),
    '20.57',
    '2025-12-14 09:00:00'
),

-- 收缩压
(
    4,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.systolic'),
    '128',
    '2025-12-14 09:05:00'
),

-- 舒张压
(
    5,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'vital.bp.diastolic'),
    '83',
    '2025-12-14 09:05:00'
),

-- 血红蛋白
(
    6,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.blood.hgb'),
    '158',
    '2025-12-14 10:00:00'
),

-- 白细胞
(
    7,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.blood.wbc'),
    '7.03',
    '2025-12-14 10:00:00'
),

-- ALT
(
    8,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.alt'),
    '24.3',
    '2025-12-14 10:30:00'
),

-- 空腹血糖
(
    9,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.biochem.fpg'),
    '5.07',
    '2025-12-14 10:30:00'
),

-- 幽门螺杆菌抗体
(
    10,
    1,
    (SELECT metric_id FROM metric WHERE metric_code = 'lab.hp.antibody'),
    'negative',
    '2025-12-14 11:00:00'
);
