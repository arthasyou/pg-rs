-- ===============================
-- Subjects 初始化（10 条）
-- ===============================

INSERT INTO subject (subject_id, subject_type)
VALUES
(1, 'person'),              -- 主体：你自己
(2, 'person'),              -- 家庭成员 A
(3, 'person'),              -- 家庭成员 B
(4, 'person'),              -- 家庭成员 C

(5, 'anonymous_person'),    -- 历史体检导入
(6, 'anonymous_person'),    -- 第三方数据

(7, 'device'),              -- 血压计
(8, 'device'),              -- 血糖仪

(9, 'test_subject'),        -- 系统测试
(10, 'test_subject');       -- 系统测试
