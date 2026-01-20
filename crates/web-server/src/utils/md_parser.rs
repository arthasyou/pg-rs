use serde_json::{json, Value as JsonValue};

/// 解析 Markdown 文件为 JSON 格式
pub fn parse_markdown(content: &str) -> JsonValue {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = json!({
        "title": null,
        "content": null,
        "sections": [],
        "metadata": {}
    });

    let mut current_section: Option<String> = None;
    let mut current_content = String::new();
    let mut sections = Vec::new();

    for line in lines {
        // 处理标题 (# Title)
        if line.starts_with("# ") {
            // 如果有之前的 section，保存它
            if let Some(section_name) = current_section.take() {
                sections.push(json!({
                    "title": section_name,
                    "content": current_content.trim().to_string()
                }));
                current_content.clear();
            }

            let title = line.trim_start_matches("# ").trim();
            if result["title"].is_null() {
                result["title"] = json!(title);
            }
        }
        // 处理二级标题 (## Section)
        else if line.starts_with("## ") {
            // 保存前一个 section
            if let Some(section_name) = current_section.take() {
                sections.push(json!({
                    "title": section_name,
                    "content": current_content.trim().to_string()
                }));
                current_content.clear();
            }

            let section_title = line.trim_start_matches("## ").trim();
            current_section = Some(section_title.to_string());
        }
        // 处理三级标题及更深层级 (### 等)
        else if line.starts_with("### ") {
            let subsection = line.trim_start_matches("### ").trim();
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(&format!("**{}**", subsection));
        }
        // 处理代码块
        else if line.starts_with("```") {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str("```");
        }
        // 处理列表项
        else if line.trim().starts_with("- ") || line.trim().starts_with("* ") {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            let item = line.trim_start_matches("- ").trim_start_matches("* ").trim();
            current_content.push_str(&format!("- {}", item));
        }
        // 处理数字列表
        else if line.trim().starts_with(|c: char| c.is_ascii_digit())
            && line.contains(". ")
        {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line.trim());
        }
        // 其他内容行
        else if !line.trim().is_empty() {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    // 保存最后一个 section
    if let Some(section_name) = current_section {
        sections.push(json!({
            "title": section_name,
            "content": current_content.trim().to_string()
        }));
    } else if !current_content.is_empty() {
        // 如果没有 section，作为主内容
        if result["content"].is_null() {
            result["content"] = json!(current_content.trim().to_string());
        }
    }

    if !sections.is_empty() {
        result["sections"] = json!(sections);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown() {
        let md = r#"# My Document

This is the main content.

## Section 1

Content of section 1.

## Section 2

Content of section 2.
"#;

        let result = parse_markdown(md);

        assert_eq!(result["title"], "My Document");
        assert!(result["sections"].is_array());
        assert_eq!(result["sections"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_markdown_with_lists() {
        let md = r#"# API Documentation

- Item 1
- Item 2
- Item 3
"#;

        let result = parse_markdown(md);

        assert_eq!(result["title"], "API Documentation");
        assert!(result["content"].as_str().unwrap().contains("- Item 1"));
    }
}
