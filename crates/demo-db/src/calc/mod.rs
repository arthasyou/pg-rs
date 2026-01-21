use std::collections::HashMap;

use pg_tables::table::metric::dto::MetricId;
use serde_json::Value;

use crate::{Error, Result};

type CalcFn = fn(inputs: &HashMap<MetricId, f64>) -> Result<f64>;

pub fn get_calc(calc_key: &str) -> Option<CalcFn> {
    match calc_key {
        "tyg_v1" => Some(calc_tyg_v1),
        _ => None,
    }
}

fn calc_tyg_v1(inputs: &HashMap<MetricId, f64>) -> Result<f64> {
    let tg = inputs
        .get(&MetricId(16))
        .ok_or(Error::internal("missing TG"))?;
    let fpg = inputs
        .get(&MetricId(18))
        .ok_or(Error::internal("missing FPG"))?;

    // 占位实现，后面再换正式公式
    Ok((tg * fpg).ln())
}

pub fn parse_inputs(inputs: &Value) -> Result<HashMap<MetricId, f64>> {
    let obj = inputs
        .as_object()
        .ok_or(Error::internal("inputs is not a JSON object"))?;

    let mut map = HashMap::new();

    for (k, v) in obj {
        // 1. key: "16" -> MetricId(16)
        let metric_id: i64 = k
            .parse()
            .map_err(|_| Error::internal("invalid metric_id key"))?;
        let metric_id = MetricId(metric_id);

        // 2. value: "2.31" -> f64
        let value_str = v.as_str().ok_or(Error::internal("value is not string"))?;
        let value: f64 = value_str
            .parse()
            .map_err(|_| Error::internal("invalid numeric value"))?;

        map.insert(metric_id, value);
    }

    Ok(map)
}
