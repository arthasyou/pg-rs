在 db_deme/src/api/medical.rs这个文件
添加一个新的功能

> 请实现一个业务查询函数，用于**计算综合指标（如 `tyg_v1`）**，直接调用已有工具函数，不新增抽象。
>
> 实现步骤必须严格按下面顺序：
>
> 1. **根据 `metric_code` 查询一条 recipe**
>    - 取出 `deps`（`Vec<MetricId>`）
>    - 取出 `calc_key`
> 2. **调用现有函数**
>
>    ```rust
>
>    ```
>
> query_observation_by_metrics(
> subject_id,
> deps,
> range,
> )
>
> ````
>    得到：
>    ```rust
> Vec<ObservationInputs>
> ````
>
> 3. **对每一条 `ObservationInputs`：**
>    - 调用 `parse_inputs(&row.inputs)` 得到 `HashMap<MetricId, f64>`
>    - 用 `get_calc(calc_key)` 取得计算函数
>    - 执行计算，得到 `f64` 结果
> 4. **返回结果**
>    - 返回一个按 `observed_at` 排序的列表
>    - 每个元素包含：
>      - `observed_at`
>      - `value`（综合指标计算结果）
>
> 约束：
>
> - 不写 SQL
> - 不修改 `query_observation_by_metrics`
> - 不修改 `parse_inputs / get_calc / calc_tyg_v1`
> - 不引入新概念或新结构
>
> 只输出完整函数代码。
