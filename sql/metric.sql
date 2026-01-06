
-- ======================================================
-- Vital Signs / 一般体征
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('vital.height', '身高', 'cm', 'number', 'line_chart'),
('vital.weight', '体重', 'kg', 'number', 'line_chart'),
('vital.bmi', '体重指数', 'kg/m2', 'number', 'line_chart'),
('vital.bp.systolic', '收缩压', 'mmHg', 'number', 'line_chart'),
('vital.bp.diastolic', '舒张压', 'mmHg', 'number', 'line_chart');

-- ======================================================
-- Blood Routine / 血常规
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.blood.rbc', '红细胞计数', '10^12/L', 'number', 'line_chart'),
('lab.blood.hgb', '血红蛋白', 'g/L', 'number', 'line_chart'),
('lab.blood.hct', '红细胞压积', '%', 'number', 'line_chart'),
('lab.blood.wbc', '白细胞计数', '10^9/L', 'number', 'line_chart'),
('lab.blood.plt', '血小板计数', '10^9/L', 'number', 'line_chart');

-- ======================================================
-- Biochemistry / 生化
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.biochem.alt', '丙氨酸氨基转移酶', 'U/L', 'number', 'line_chart'),
('lab.biochem.tp', '总蛋白', 'g/L', 'number', 'line_chart'),
('lab.biochem.alb', '白蛋白', 'g/L', 'number', 'line_chart'),
('lab.biochem.tbili', '总胆红素', 'umol/L', 'number', 'line_chart'),
('lab.biochem.tc', '总胆固醇', 'mmol/L', 'number', 'line_chart'),
('lab.biochem.tg', '甘油三酯', 'mmol/L', 'number', 'line_chart'),
('lab.biochem.ua', '尿酸', 'umol/L', 'number', 'line_chart'),
('lab.biochem.fpg', '空腹血糖', 'mmol/L', 'number', 'line_chart');

-- ======================================================
-- Urine Routine / 尿常规（核心）
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.urine.sg', '尿比重', '', 'number', 'line_chart'),
('lab.urine.ph', '尿PH值', '', 'number', 'line_chart'),
('lab.urine.protein', '尿蛋白', '', 'enum', 'value_list'),
('lab.urine.glucose', '尿糖', '', 'enum', 'value_list');

-- ======================================================
-- Infection / 传染与检测
-- ======================================================

INSERT INTO metric (metric_code, metric_name, unit, value_type, visualization)
VALUES
('lab.hp.antibody', '幽门螺杆菌抗体', '', 'enum', 'value_list');