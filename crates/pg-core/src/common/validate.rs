// Re-export validator types
pub use validator::{Validate, ValidationError, ValidationErrors};

/// Validate a struct that implements the Validate trait
pub fn validate_struct<T: Validate>(value: &T) -> Result<(), ValidationErrors> {
    value.validate()
}

/// Format validation errors into a human-readable string
pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();

    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = error
                .message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| format!("Validation failed for field: {}", field));
            messages.push(message);
        }
    }

    messages.join("; ")
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::*;

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min = 1, max = 10))]
        name: String,
        #[validate(range(min = 0, max = 100))]
        age: i32,
    }

    #[test]
    fn test_validate_struct_success() {
        let test = TestStruct {
            name: "test".to_string(),
            age: 25,
        };

        assert!(validate_struct(&test).is_ok());
    }

    #[test]
    fn test_validate_struct_failure() {
        let test = TestStruct {
            name: "".to_string(), // Too short
            age: 150,             // Out of range
        };

        let result = validate_struct(&test);
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("name"));
            assert!(errors.field_errors().contains_key("age"));
        }
    }

    #[test]
    fn test_format_validation_errors() {
        let test = TestStruct {
            name: "".to_string(),
            age: 150,
        };

        if let Err(errors) = validate_struct(&test) {
            let formatted = format_validation_errors(&errors);
            assert!(!formatted.is_empty());
        }
    }
}
