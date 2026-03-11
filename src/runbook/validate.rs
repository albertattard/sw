use super::{ValidationIssue, ValidationResult};
use serde_json::{Map, Value};

fn push_error(
    errors: &mut Vec<ValidationIssue>,
    path: impl Into<String>,
    message: impl Into<String>,
) {
    errors.push(ValidationIssue {
        path: path.into(),
        message: message.into(),
    });
}

fn as_object<'a>(
    value: &'a Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
) -> Option<&'a Map<String, Value>> {
    match value.as_object() {
        Some(object) => Some(object),
        None => {
            push_error(errors, path, "must be an object");
            None
        }
    }
}

fn as_array<'a>(
    value: &'a Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
) -> Option<&'a [Value]> {
    match value.as_array() {
        Some(array) => Some(array),
        None => {
            push_error(errors, path, "must be an array");
            None
        }
    }
}

fn require_string(
    object: &Map<String, Value>,
    key: &str,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
) {
    match object.get(key).and_then(Value::as_str) {
        Some(_) => {}
        None => push_error(errors, format!("{path}.{key}"), "must be a string"),
    }
}

fn validate_string_array(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    if let Some(items) = as_array(value, path, errors) {
        for (index, item) in items.iter().enumerate() {
            if !item.is_string() {
                push_error(errors, format!("{path}[{index}]"), "must be a string");
            }
        }
    }
}

fn validate_output(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(object) = as_object(value, path, errors) else {
        return;
    };

    for key in object.keys() {
        if key != "caption" {
            push_error(
                errors,
                format!("{path}.{key}"),
                "is not a supported output property",
            );
        }
    }

    match object.get("caption") {
        Some(Value::String(_)) => {}
        Some(caption) if caption.is_array() => {
            validate_string_array(caption, &format!("{path}.caption"), errors);
        }
        Some(_) => push_error(
            errors,
            format!("{path}.caption"),
            "must be a string or array of strings",
        ),
        None => push_error(errors, format!("{path}.caption"), "is required"),
    }
}

fn validate_entry(value: &Value, index: usize, errors: &mut Vec<ValidationIssue>) {
    let path = format!("entries[{index}]");
    let Some(object) = as_object(value, &path, errors) else {
        return;
    };

    let entry_type = match object.get("type").and_then(Value::as_str) {
        Some(entry_type) => entry_type,
        None => {
            push_error(errors, format!("{path}.type"), "must be a string");
            return;
        }
    };

    match entry_type {
        "Heading" => {
            require_string(object, "level", &path, errors);
            require_string(object, "title", &path, errors);
        }
        "Markdown" => match object.get("contents") {
            Some(contents) => validate_string_array(contents, &format!("{path}.contents"), errors),
            None => push_error(errors, format!("{path}.contents"), "is required"),
        },
        "Command" => {
            match object.get("commands") {
                Some(commands) => {
                    validate_string_array(commands, &format!("{path}.commands"), errors)
                }
                None => push_error(errors, format!("{path}.commands"), "is required"),
            }

            if let Some(indent) = object.get("indent") {
                if !indent.is_i64() && !indent.is_u64() {
                    push_error(errors, format!("{path}.indent"), "must be an integer");
                }
            }

            if let Some(output) = object.get("output") {
                validate_output(output, &format!("{path}.output"), errors);
            }
        }
        _ => push_error(
            errors,
            format!("{path}.type"),
            format!("unsupported entry type `{entry_type}`"),
        ),
    }
}

pub fn validate(runbook: &Value) -> ValidationResult {
    let mut errors = Vec::new();
    let warnings = Vec::new();

    let Some(object) = as_object(runbook, "$", &mut errors) else {
        return ValidationResult {
            schema_version: "1",
            valid: false,
            errors,
            warnings,
        };
    };

    for key in object.keys() {
        if key != "entries" {
            push_error(
                &mut errors,
                format!("$.{key}"),
                "unknown top-level property",
            );
        }
    }

    match object.get("entries") {
        Some(entries) => {
            if let Some(items) = as_array(entries, "$.entries", &mut errors) {
                if items.is_empty() {
                    push_error(&mut errors, "$.entries", "must not be empty");
                }

                for (index, entry) in items.iter().enumerate() {
                    validate_entry(entry, index, &mut errors);
                }
            }
        }
        None => push_error(&mut errors, "$.entries", "is required"),
    }

    ValidationResult {
        schema_version: "1",
        valid: errors.is_empty(),
        errors,
        warnings,
    }
}
