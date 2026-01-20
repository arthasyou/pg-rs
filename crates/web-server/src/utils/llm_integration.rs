use model_gateway_rs::{
    model::llm::{ChatMessage, LlmInput},
    sdk::{ModelSDK, ollama::OllamaSdk},
};
use serde_json::Value as JsonValue;

/// 使用 LLM 从健康检查文本中提取医疗指标
pub async fn extract_health_metrics_with_llm(
    content: &str,
    llm_base_url: &str,
    llm_model: &str,
) -> Result<JsonValue, String> {
    let prompt = format!(
        r#"你是一个医疗信息提取专家。请从以下健康检查文档中提取所有医疗指标，并返回JSON格式的结构化数据。

健康检查文档：
{}

请提取以下信息，并只返回有效的JSON，不要返回其他文本：

1. 患者基本信息 (patient_info): 包含姓名、年龄、性别、检查日期
2. 医疗指标列表 (metrics): 每个指标包含：
   - metric_code: 指标代码 (英文，如 blood_glucose, blood_pressure 等)
   - value: 指标值（包含单位）
   - reference_range: 参考范围（如果有）
   - is_abnormal: 是否异常 (boolean)
   - abnormality_note: 异常说明（如果异常）

3. 诊断和结论 (diagnoses): 列表格式
4. 健康建议 (recommendations): 列表格式

只提取与医疗相关的内容，忽略营销信息、法律免责声明等无关内容。

返回的JSON格式：
{{
  "patient_info": {{"name": "...", "age": "...", "gender": "...", "check_date": "..."}},
  "metrics": [
    {{"metric_code": "...", "value": "...", "reference_range": "...", "is_abnormal": false, "abnormality_note": null}}
  ],
  "diagnoses": ["..."],
  "recommendations": ["..."]
}}
"#,
        content
    );

    // 创建 Ollama SDK - 确保 base_url 以 /api/ 结尾
    let base_url_with_api = if llm_base_url.ends_with("/api/") {
        llm_base_url.to_string()
    } else if llm_base_url.ends_with("/") {
        format!("{}api/", llm_base_url)
    } else {
        format!("{}/api/", llm_base_url)
    };

    let sdk = OllamaSdk::new(&base_url_with_api, llm_model)
        .map_err(|e| format!("Failed to create Ollama SDK: {}", e))?;

    let input = LlmInput {
        messages: vec![ChatMessage::user(&prompt)],
        max_tokens: Some(8000),
    };

    let output = sdk
        .chat_once(input)
        .await
        .map_err(|e| format!("LLM request failed: {}", e))?;

    let response_text = output.message.content;

    tracing::debug!("LLM Response (raw): {}", response_text);

    // 尝试从响应中解析 JSON
    // LLM 可能会返回额外的文本，我们需要提取 JSON 部分
    if let Ok(parsed) = serde_json::from_str::<JsonValue>(&response_text) {
        return Ok(parsed);
    }

    // 尝试找到 JSON 块
    if let Some(start) = response_text.find('{') {
        if let Some(end) = response_text.rfind('}') {
            if end > start {
                let json_str = &response_text[start..=end];
                if let Ok(parsed) = serde_json::from_str::<JsonValue>(json_str) {
                    return Ok(parsed);
                }
            }
        }
    }

    Err(format!(
        "Failed to parse JSON from LLM response: {}",
        response_text
    ))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_extraction() {
        let text = r#"Some text before {"key": "value"} some text after"#;
        if let Some(start) = text.find('{') {
            if let Some(end) = text.rfind('}') {
                let json_str = &text[start..=end];
                let parsed = serde_json::from_str::<JsonValue>(json_str);
                assert!(parsed.is_ok());
            }
        }
    }
}
