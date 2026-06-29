use super::{RenderError, ValidationIssue};
use serde_json::{Map, Value};

const OS_VALUES: [&str; 3] = ["macos", "linux", "windows"];

pub(crate) fn command_should_execute(entry: &Value) -> Result<bool, RenderError> {
    let Some(condition) = entry.get("execute_when") else {
        return Ok(true);
    };
    let object = condition.as_object().ok_or_else(|| {
        RenderError::Operational("Command execute_when must be an object".to_string())
    })?;

    let fact = object.get("fact").and_then(Value::as_str).ok_or_else(|| {
        RenderError::Operational("Command execute_when.fact must be a string".to_string())
    })?;
    if fact != "os" {
        return Err(RenderError::Operational(
            "Command execute_when.fact must be `os`".to_string(),
        ));
    }

    let expected = object
        .get("equals")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RenderError::Operational("Command execute_when.equals must be a string".to_string())
        })?;
    if !OS_VALUES.contains(&expected) {
        return Err(RenderError::Operational(format!(
            "Command execute_when.equals must be one of `{}`",
            OS_VALUES.join("`, `")
        )));
    }

    Ok(expected == current_os())
}

pub(crate) fn has_execute_when(entry: &Map<String, Value>) -> bool {
    entry.contains_key("execute_when")
}

pub(crate) fn validate_execute_when(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(object) = value.as_object() else {
        errors.push(ValidationIssue {
            path: path.to_string(),
            message: "must be an object".to_string(),
        });
        return;
    };

    for key in object.keys() {
        if key != "fact" && key != "equals" {
            errors.push(ValidationIssue {
                path: format!("{path}.{key}"),
                message: "is not a supported execute_when property".to_string(),
            });
        }
    }

    match object.get("fact").and_then(Value::as_str) {
        Some("os") => {}
        Some(_) => errors.push(ValidationIssue {
            path: format!("{path}.fact"),
            message: "must be `os`".to_string(),
        }),
        None => errors.push(ValidationIssue {
            path: format!("{path}.fact"),
            message: "must be a string".to_string(),
        }),
    }

    match object.get("equals").and_then(Value::as_str) {
        Some(value) if OS_VALUES.contains(&value) => {}
        Some(_) => errors.push(ValidationIssue {
            path: format!("{path}.equals"),
            message: format!("must be one of `{}`", OS_VALUES.join("`, `")),
        }),
        None => errors.push(ValidationIssue {
            path: format!("{path}.equals"),
            message: "must be a string".to_string(),
        }),
    }
}

fn current_os() -> &'static str {
    std::env::consts::OS
}
