// Re-export validator crate for convenience
pub use validator::{Validate, ValidationError, ValidationErrors};

/// Helper function to validate a struct and return a formatted error message
pub fn validate_struct<T: Validate>(value: &T) -> Result<(), String> {
    value.validate().map_err(|e| format_validation_errors(&e))
}

/// Format validation errors into a readable string
pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();

    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = error
                .message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| format!("Invalid value for {}", field));
            messages.push(format!("{}: {}", field, message));
        }
    }

    messages.join(", ")
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::*;

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 1, message = "name is required"))]
        name: String,

        #[validate(length(min = 3, max = 50))]
        username: String,

        #[validate(email)]
        email: String,

        #[validate(range(min = 0, max = 120))]
        age: i32,
    }

    #[test]
    fn test_valid_struct() {
        let test = TestStruct {
            name: "John".to_string(),
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
            age: 25,
        };

        assert!(validate_struct(&test).is_ok());
    }

    #[test]
    fn test_invalid_struct() {
        let test = TestStruct {
            name: "".to_string(),
            username: "ab".to_string(),
            email: "invalid-email".to_string(),
            age: 150,
        };

        let result = validate_struct(&test);
        assert!(result.is_err());

        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("name"));
    }

    #[test]
    fn test_format_errors() {
        let test = TestStruct {
            name: "".to_string(),
            username: "ab".to_string(),
            email: "john@example.com".to_string(),
            age: 25,
        };

        if let Err(errors) = test.validate() {
            let formatted = format_validation_errors(&errors);
            assert!(!formatted.is_empty());
        }
    }
}
