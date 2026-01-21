-- ======================================================
-- Vital Signs / 一般体征
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('vital.height', '身高', 'cm', 'float', 'line_chart'),
('vital.weight', '体重', 'kg', 'float', 'line_chart'),
('vital.bmi', '体重指数', 'kg/m2', 'float', 'line_chart'),
('vital.bp.systolic', '收缩压', 'mmHg', 'float', 'line_chart'),
('vital.bp.diastolic', '舒张压', 'mmHg', 'float', 'line_chart');

-- ======================================================
-- Blood Routine / 血常规
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.blood.rbc', '红细胞计数', '10^12/L', 'float', 'line_chart'),
('lab.blood.hgb', '血红蛋白', 'g/L', 'float', 'line_chart'),
('lab.blood.hct', '红细胞压积', '%', 'float', 'line_chart'),
('lab.blood.wbc', '白细胞计数', '10^9/L', 'float', 'line_chart'),
('lab.blood.plt', '血小板计数', '10^9/L', 'float', 'line_chart');

-- ======================================================
-- Biochemistry / 生化
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.biochem.alt', '丙氨酸氨基转移酶', 'U/L', 'float', 'line_chart'),
('lab.biochem.tp', '总蛋白', 'g/L', 'float', 'line_chart'),
('lab.biochem.alb', '白蛋白', 'g/L', 'float', 'line_chart'),
('lab.biochem.tbili', '总胆红素', 'umol/L', 'float', 'line_chart'),
('lab.biochem.tc', '总胆固醇', 'mmol/L', 'float', 'line_chart'),
('lab.biochem.tg', '甘油三酯', 'mmol/L', 'float', 'line_chart'),
('lab.biochem.ua', '尿酸', 'umol/L', 'float', 'line_chart'),
('lab.biochem.fpg', '空腹血糖', 'mmol/L', 'float', 'line_chart');

-- ======================================================
-- Urine Routine / 尿常规（核心）
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.urine.sg', '尿比重', '', 'float', 'line_chart'),
('lab.urine.ph', '尿PH值', '', 'float', 'line_chart'),
('lab.urine.protein', '尿蛋白', '', 'text', 'value_list'),
('lab.urine.glucose', '尿糖', '', 'text', 'value_list');

-- ======================================================
-- Infection / 传染与检测
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.hp.antibody', '幽门螺杆菌抗体', '', 'text', 'value_list');

-- ======================================================
-- recipe / 综合指标
-- ======================================================
-- ======================================================
-- metric / 综合指标（Recipe）
-- ======================================================

INSERT INTO metric (
    metric_code,
    metric_name,
    unit,
    value_type,
    visualization,
    kind
)
VALUES (
    'calc.tyg',
    '胰岛素抵抗指数（TyG）',
    NULL,
    'float',
    'line',
    'Derived'
);


