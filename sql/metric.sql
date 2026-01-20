-- ===============================
-- Vital Signs / 一般体征
-- ===============================

INSERT INTO metric (metric_code, metric_name, unit, value_type)
VALUES
('vital.height', '身高', 'cm', 'number'),
('vital.weight', '体重', 'kg', 'number'),
('vital.bmi', '体重指数', 'kg/m2', 'number'),
('vital.bp.systolic', '收缩压', 'mmHg', 'number'),
('vital.bp.diastolic', '舒张压', 'mmHg', 'number');

-- ===============================
-- Blood Routine / 血常规
-- ===============================

INSERT INTO metric (metric_code, metric_name, unit, value_type)
VALUES
('lab.blood.rbc', '红细胞计数', '10^12/L', 'number'),
('lab.blood.hgb', '血红蛋白', 'g/L', 'number'),
('lab.blood.hct', '红细胞压积', '%', 'number'),
('lab.blood.wbc', '白细胞计数', '10^9/L', 'number'),
('lab.blood.plt', '血小板计数', '10^9/L', 'number');

-- ===============================
-- Biochemistry / 生化
-- ===============================

INSERT INTO metric (metric_code, metric_name, unit, value_type)
VALUES
('lab.biochem.alt', '丙氨酸氨基转移酶', 'U/L', 'number'),
('lab.biochem.tp', '总蛋白', 'g/L', 'number'),
('lab.biochem.alb', '白蛋白', 'g/L', 'number'),
('lab.biochem.tbili', '总胆红素', 'umol/L', 'number'),
('lab.biochem.tc', '总胆固醇', 'mmol/L', 'number'),
('lab.biochem.tg', '甘油三酯', 'mmol/L', 'number'),
('lab.biochem.ua', '尿酸', 'umol/L', 'number'),
('lab.biochem.fpg', '空腹血糖', 'mmol/L', 'number');

-- ===============================
-- Urine Routine / 尿常规（核心）
-- ===============================

INSERT INTO metric (metric_code, metric_name, unit, value_type)
VALUES
('lab.urine.sg', '尿比重', '', 'number'),
('lab.urine.ph', '尿PH值', '', 'number'),
('lab.urine.protein', '尿蛋白', '', 'enum'),
('lab.urine.glucose', '尿糖', '', 'enum');

-- ===============================
-- Infection / 传染与检测
-- ===============================

INSERT INTO metric (metric_code, metric_name, unit, value_type)
VALUES
('lab.hp.antibody', '幽门螺杆菌抗体', '', 'enum');
