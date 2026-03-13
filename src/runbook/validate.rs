use super::{ValidationIssue, ValidationResult};
use regex::Regex;
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::time::Duration;

const CAPTURE_NAME_PATTERN: &str = r"^[A-Za-z_][A-Za-z0-9_]*$";
const CAPTURE_REFERENCE_PATTERN: &str =
    r"@@\{([A-Za-z_][A-Za-z0-9_]*)\}|@\{([A-Za-z_][A-Za-z0-9_]*)\}";

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

fn validate_output_with_context(
    value: &Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    global_datetime_anchor_ids: &mut HashSet<String>,
    global_capture_names: &mut HashSet<String>,
    available_capture_names: &HashSet<String>,
) {
    let Some(object) = as_object(value, path, errors) else {
        return;
    };

    for key in object.keys() {
        if key != "caption"
            && key != "content_type"
            && key != "trim_trailing_whitespace"
            && key != "rewrite"
        {
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
        None => {}
    }

    match object.get("content_type").and_then(Value::as_str) {
        Some("text" | "json" | "xml") => {}
        Some(_) => push_error(
            errors,
            format!("{path}.content_type"),
            "must be one of `text`, `json`, or `xml`",
        ),
        None => {}
    }

    match object.get("trim_trailing_whitespace") {
        Some(Value::Bool(_)) => {}
        Some(_) => push_error(
            errors,
            format!("{path}.trim_trailing_whitespace"),
            "must be a boolean",
        ),
        None => {}
    }

    if let Some(rewrite) = object.get("rewrite") {
        validate_rewrite_rules(
            rewrite,
            &format!("{path}.rewrite"),
            errors,
            global_datetime_anchor_ids,
            global_capture_names,
            available_capture_names,
        );
    }
}

fn validate_capture(
    value: &Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    global_capture_names: &mut HashSet<String>,
) {
    let Some(captures) = as_array(value, path, errors) else {
        return;
    };

    if captures.is_empty() {
        push_error(errors, path.to_string(), "must not be empty");
    }

    let name_pattern = Regex::new(CAPTURE_NAME_PATTERN).expect("valid capture name regex");

    for (index, capture) in captures.iter().enumerate() {
        let capture_path = format!("{path}[{index}]");
        let Some(object) = as_object(capture, &capture_path, errors) else {
            continue;
        };

        for key in object.keys() {
            if key != "name" && key != "source" && key != "stage" && key != "pattern" {
                push_error(
                    errors,
                    format!("{capture_path}.{key}"),
                    "is not a supported capture property",
                );
            }
        }

        match object.get("name").and_then(Value::as_str) {
            Some(name) => {
                if !name_pattern.is_match(name) {
                    push_error(
                        errors,
                        format!("{capture_path}.name"),
                        "must be a valid identifier",
                    );
                } else if !global_capture_names.insert(name.to_string()) {
                    push_error(
                        errors,
                        format!("{capture_path}.name"),
                        format!("duplicate capture name `{name}`"),
                    );
                }
            }
            None => push_error(errors, format!("{capture_path}.name"), "must be a string"),
        }

        match object.get("source").and_then(Value::as_str) {
            Some("stdout") => {}
            Some(_) => push_error(errors, format!("{capture_path}.source"), "must be `stdout`"),
            None => push_error(errors, format!("{capture_path}.source"), "must be a string"),
        }

        match object.get("stage").and_then(Value::as_str) {
            Some("raw" | "rewritten") => {}
            Some(_) => push_error(
                errors,
                format!("{capture_path}.stage"),
                "must be `raw` or `rewritten`",
            ),
            None => push_error(errors, format!("{capture_path}.stage"), "must be a string"),
        }

        match object.get("pattern").and_then(Value::as_str) {
            Some(pattern) => {
                if let Err(err) = Regex::new(pattern) {
                    push_error(
                        errors,
                        format!("{capture_path}.pattern"),
                        format!("must be a valid regex: {err}"),
                    );
                }
            }
            None => push_error(
                errors,
                format!("{capture_path}.pattern"),
                "must be a string",
            ),
        }
    }
}

fn validate_capture_references(
    strings: &Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    available_capture_names: &HashSet<String>,
) {
    let Some(strings) = as_array(strings, path, errors) else {
        return;
    };

    let reference_pattern =
        Regex::new(CAPTURE_REFERENCE_PATTERN).expect("valid capture reference regex");

    for (index, value) in strings.iter().enumerate() {
        let Some(value) = value.as_str() else {
            continue;
        };

        validate_capture_references_in_string(
            value,
            &format!("{path}[{index}]"),
            errors,
            available_capture_names,
            &reference_pattern,
        );
    }
}

fn register_rewrite_generated_capture_names(
    output: &Value,
    available_capture_names: &mut HashSet<String>,
) {
    let Some(rules) = output.get("rewrite").and_then(Value::as_array) else {
        return;
    };

    for rule in rules {
        let Some(capture_as) = rule.get("capture_as").and_then(Value::as_str) else {
            continue;
        };

        available_capture_names.insert(format!("{capture_as}_original"));
        available_capture_names.insert(format!("{capture_as}_rewritten"));
    }
}

fn validate_capture_references_in_string(
    value: &str,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    available_capture_names: &HashSet<String>,
    reference_pattern: &Regex,
) {
    for captures in reference_pattern.captures_iter(value) {
        let Some(name) = captures.get(2) else {
            continue;
        };

        if !available_capture_names.contains(name.as_str()) {
            push_error(
                errors,
                path.to_string(),
                format!(
                    "references capture variable before it is defined: `@{{{}}}`",
                    name.as_str()
                ),
            );
        }
    }
}

fn validate_rewrite_rules(
    value: &Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    global_datetime_anchor_ids: &mut HashSet<String>,
    global_capture_names: &mut HashSet<String>,
    available_capture_names: &HashSet<String>,
) {
    let Some(rules) = as_array(value, path, errors) else {
        return;
    };

    if rules.is_empty() {
        push_error(errors, path.to_string(), "must not be empty");
    }
    for (index, rule) in rules.iter().enumerate() {
        validate_rewrite_rule(
            rule,
            &format!("{path}[{index}]"),
            errors,
            global_datetime_anchor_ids,
            global_capture_names,
            available_capture_names,
        );
    }
}

fn validate_rewrite_rule(
    value: &Value,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    global_datetime_anchor_ids: &mut HashSet<String>,
    global_capture_names: &mut HashSet<String>,
    available_capture_names: &HashSet<String>,
) {
    let Some(object) = as_object(value, path, errors) else {
        return;
    };

    let rule_type = match object.get("type").and_then(Value::as_str) {
        Some(rule_type) => rule_type,
        None => {
            push_error(errors, format!("{path}.type"), "must be a string");
            return;
        }
    };

    match rule_type {
        "replace" => {
            for key in object.keys() {
                if key != "type" && key != "pattern" && key != "replacement" && key != "capture_as"
                {
                    push_error(
                        errors,
                        format!("{path}.{key}"),
                        "is not a supported replace rewrite property",
                    );
                }
            }

            require_string(object, "pattern", path, errors);
            require_string(object, "replacement", path, errors);
            validate_rewrite_capture_as(object, path, errors, global_capture_names);
            if let Some(pattern) = object.get("pattern").and_then(Value::as_str) {
                let reference_pattern =
                    Regex::new(CAPTURE_REFERENCE_PATTERN).expect("valid capture reference regex");
                validate_capture_references_in_string(
                    pattern,
                    &format!("{path}.pattern"),
                    errors,
                    available_capture_names,
                    &reference_pattern,
                );
            }
            if let Some(replacement) = object.get("replacement").and_then(Value::as_str) {
                let reference_pattern =
                    Regex::new(CAPTURE_REFERENCE_PATTERN).expect("valid capture reference regex");
                validate_capture_references_in_string(
                    replacement,
                    &format!("{path}.replacement"),
                    errors,
                    available_capture_names,
                    &reference_pattern,
                );
            }
        }
        "keep_between" => {
            for key in object.keys() {
                if key != "type"
                    && key != "start"
                    && key != "end"
                    && key != "start_offset"
                    && key != "end_offset"
                    && key != "show_trim_markers"
                {
                    push_error(
                        errors,
                        format!("{path}.{key}"),
                        "is not a supported keep_between rewrite property",
                    );
                }
            }

            require_string(object, "start", path, errors);
            require_string(object, "end", path, errors);

            if let Some(start_offset) = object.get("start_offset")
                && !start_offset.is_i64()
                && !start_offset.is_u64()
            {
                push_error(errors, format!("{path}.start_offset"), "must be an integer");
            }

            if let Some(end_offset) = object.get("end_offset")
                && !end_offset.is_i64()
                && !end_offset.is_u64()
            {
                push_error(errors, format!("{path}.end_offset"), "must be an integer");
            }

            if let Some(show_trim_markers) = object.get("show_trim_markers")
                && !show_trim_markers.is_boolean()
            {
                push_error(
                    errors,
                    format!("{path}.show_trim_markers"),
                    "must be a boolean",
                );
            }
        }
        "datetime_shift" => {
            for key in object.keys() {
                if key != "type"
                    && key != "pattern"
                    && key != "base"
                    && key != "format"
                    && key != "id"
                    && key != "use"
                    && key != "custom_format"
                    && key != "capture_as"
                {
                    push_error(
                        errors,
                        format!("{path}.{key}"),
                        "is not a supported datetime_shift rewrite property",
                    );
                }
            }

            let has_pattern = object.get("pattern").is_some();
            let has_format = object.get("format").is_some();
            let has_custom_format = object.get("custom_format").is_some();
            let has_id = object.get("id").is_some();
            let has_use = object.get("use").is_some();

            if has_pattern == has_format {
                push_error(
                    errors,
                    path.to_string(),
                    "must include exactly one of `pattern` or `format`",
                );
            }

            if has_format && has_custom_format {
                push_error(
                    errors,
                    path.to_string(),
                    "must not include both `format` and `custom_format`",
                );
            }

            if has_pattern && !has_custom_format {
                push_error(
                    errors,
                    path.to_string(),
                    "must include `custom_format` when `pattern` is used",
                );
            }

            if has_custom_format && !has_pattern {
                push_error(
                    errors,
                    path.to_string(),
                    "`custom_format` requires `pattern`",
                );
            }

            if has_id && has_use {
                push_error(
                    errors,
                    path.to_string(),
                    "must not include both `id` and `use`",
                );
            }

            if has_use && object.get("base").is_some() {
                push_error(
                    errors,
                    path.to_string(),
                    "must not include `base` when `use` is declared",
                );
            }

            if let Some(pattern) = object.get("pattern")
                && !pattern.is_string()
            {
                push_error(errors, format!("{path}.pattern"), "must be a string");
            }

            match object.get("format").and_then(Value::as_str) {
                Some("rfc3339" | "rfc1123") => {}
                Some(_) => push_error(
                    errors,
                    format!("{path}.format"),
                    "must be `rfc3339` or `rfc1123`",
                ),
                None => {}
            }

            if let Some(id) = object.get("id") {
                match id.as_str() {
                    Some(id) => {
                        if !global_datetime_anchor_ids.insert(id.to_string()) {
                            push_error(
                                errors,
                                format!("{path}.id"),
                                format!("duplicate datetime_shift id `{id}`"),
                            );
                        }
                    }
                    None => push_error(errors, format!("{path}.id"), "must be a string"),
                }
            }

            if let Some(use_id) = object.get("use") {
                match use_id.as_str() {
                    Some(use_id) => {
                        if !global_datetime_anchor_ids.contains(use_id) {
                            push_error(
                                errors,
                                format!("{path}.use"),
                                format!(
                                    "must reference an anchor established earlier in the runbook: `{use_id}`"
                                ),
                            );
                        }
                    }
                    None => push_error(errors, format!("{path}.use"), "must be a string"),
                }
            }

            if let Some(custom_format) = object.get("custom_format")
                && !custom_format.is_string()
            {
                push_error(errors, format!("{path}.custom_format"), "must be a string");
            }

            if let Some(base) = object.get("base")
                && !base.is_string()
            {
                push_error(errors, format!("{path}.base"), "must be a string");
            }

            validate_rewrite_capture_as(object, path, errors, global_capture_names);
        }
        _ => push_error(
            errors,
            format!("{path}.type"),
            "must be `replace`, `datetime_shift`, or `keep_between`",
        ),
    }
}

fn validate_rewrite_capture_as(
    object: &Map<String, Value>,
    path: &str,
    errors: &mut Vec<ValidationIssue>,
    global_capture_names: &mut HashSet<String>,
) {
    let Some(capture_as) = object.get("capture_as") else {
        return;
    };

    let name_pattern = Regex::new(CAPTURE_NAME_PATTERN).expect("valid capture name regex");
    let Some(capture_as) = capture_as.as_str() else {
        push_error(errors, format!("{path}.capture_as"), "must be a string");
        return;
    };

    if !name_pattern.is_match(capture_as) {
        push_error(
            errors,
            format!("{path}.capture_as"),
            "must be a valid identifier",
        );
        return;
    }

    for generated_name in [
        format!("{capture_as}_original"),
        format!("{capture_as}_rewritten"),
    ] {
        if !global_capture_names.insert(generated_name.clone()) {
            push_error(
                errors,
                format!("{path}.capture_as"),
                format!("duplicate capture name `{generated_name}`"),
            );
        }
    }
}

fn validate_assert(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(object) = as_object(value, path, errors) else {
        return;
    };

    for key in object.keys() {
        if key != "exit_code" && key != "checks" {
            push_error(
                errors,
                format!("{path}.{key}"),
                "is not a supported assert property",
            );
        }
    }

    match object.get("exit_code") {
        Some(exit_code) if exit_code.is_i64() || exit_code.is_u64() => {}
        Some(_) => push_error(errors, format!("{path}.exit_code"), "must be an integer"),
        None => {}
    }

    if let Some(checks) = object.get("checks") {
        validate_assert_checks(checks, &format!("{path}.checks"), errors);
    }

    if object.get("exit_code").is_none() && object.get("checks").is_none() {
        push_error(
            errors,
            path.to_string(),
            "must include at least one supported assertion",
        );
    }
}

fn validate_assert_checks(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(checks) = as_array(value, path, errors) else {
        return;
    };

    if checks.is_empty() {
        push_error(errors, path.to_string(), "must not be empty");
    }

    for (index, check) in checks.iter().enumerate() {
        validate_assert_check(check, &format!("{path}[{index}]"), errors);
    }
}

fn validate_assert_check(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(object) = as_object(value, path, errors) else {
        return;
    };

    for key in object.keys() {
        if key != "source" && key != "contains" {
            push_error(
                errors,
                format!("{path}.{key}"),
                "is not a supported assertion check property",
            );
        }
    }

    match object.get("source").and_then(Value::as_str) {
        Some("stdout") => {}
        Some(_) => push_error(
            errors,
            format!("{path}.source"),
            "must be `stdout` until additional assertion sources are supported",
        ),
        None => push_error(errors, format!("{path}.source"), "must be a string"),
    }

    match object.get("contains").and_then(Value::as_str) {
        Some(_) => {}
        None => push_error(errors, format!("{path}.contains"), "must be a string"),
    }
}

fn validate_entry(
    value: &Value,
    index: usize,
    errors: &mut Vec<ValidationIssue>,
    global_datetime_anchor_ids: &mut HashSet<String>,
    global_capture_names: &mut HashSet<String>,
    available_capture_names: &mut HashSet<String>,
) {
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
            Some(contents) => {
                validate_string_array(contents, &format!("{path}.contents"), errors);
                validate_capture_references(
                    contents,
                    &format!("{path}.contents"),
                    errors,
                    available_capture_names,
                );
            }
            None => push_error(errors, format!("{path}.contents"), "is required"),
        },
        "DisplayFile" => {
            require_string(object, "path", &path, errors);
        }
        "Prerequisite" => match object.get("checks") {
            Some(checks) => validate_prerequisite_checks(checks, &format!("{path}.checks"), errors),
            None => push_error(errors, format!("{path}.checks"), "is required"),
        },
        "Command" => {
            match object.get("commands") {
                Some(commands) => {
                    validate_string_array(commands, &format!("{path}.commands"), errors);
                    validate_capture_references(
                        commands,
                        &format!("{path}.commands"),
                        errors,
                        available_capture_names,
                    );
                }
                None => push_error(errors, format!("{path}.commands"), "is required"),
            }

            if let Some(indent) = object.get("indent")
                && !indent.is_i64()
                && !indent.is_u64()
            {
                push_error(errors, format!("{path}.indent"), "must be an integer");
            }

            if let Some(output) = object.get("output") {
                validate_output_with_context(
                    output,
                    &format!("{path}.output"),
                    errors,
                    global_datetime_anchor_ids,
                    global_capture_names,
                    available_capture_names,
                );
                register_rewrite_generated_capture_names(output, available_capture_names);
            }

            if let Some(assertion) = object.get("assert") {
                validate_assert(assertion, &format!("{path}.assert"), errors);
            }

            if let Some(timeout) = object.get("timeout") {
                validate_timeout(timeout, &format!("{path}.timeout"), errors);
            }

            if let Some(cleanup) = object.get("cleanup") {
                validate_string_array(cleanup, &format!("{path}.cleanup"), errors);
            }

            if let Some(capture) = object.get("capture") {
                validate_capture(
                    capture,
                    &format!("{path}.capture"),
                    errors,
                    global_capture_names,
                );

                if let Some(captures) = capture.as_array() {
                    for capture in captures {
                        if let Some(name) = capture.get("name").and_then(Value::as_str) {
                            available_capture_names.insert(name.to_string());
                        }
                    }
                }
            }
        }
        _ => push_error(
            errors,
            format!("{path}.type"),
            format!("unsupported entry type `{entry_type}`"),
        ),
    }
}

fn validate_prerequisite_checks(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(checks) = as_array(value, path, errors) else {
        return;
    };

    if checks.is_empty() {
        push_error(errors, path.to_string(), "must not be empty");
    }

    for (index, check) in checks.iter().enumerate() {
        let check_path = format!("{path}[{index}]");
        let Some(object) = as_object(check, &check_path, errors) else {
            continue;
        };

        for key in object.keys() {
            if key != "name"
                && key != "contents"
                && key != "commands"
                && key != "assert"
                && key != "help"
            {
                push_error(
                    errors,
                    format!("{check_path}.{key}"),
                    "is not a supported prerequisite property",
                );
            }
        }

        require_string(object, "name", &check_path, errors);
        match object.get("contents") {
            Some(contents) => {
                validate_string_array(contents, &format!("{check_path}.contents"), errors)
            }
            None => push_error(errors, format!("{check_path}.contents"), "is required"),
        }
        match object.get("commands") {
            Some(commands) => {
                validate_string_array(commands, &format!("{check_path}.commands"), errors)
            }
            None => push_error(errors, format!("{check_path}.commands"), "is required"),
        }

        if let Some(assertion) = object.get("assert") {
            validate_assert(assertion, &format!("{check_path}.assert"), errors);
        }

        if let Some(help) = object.get("help")
            && !help.is_string()
        {
            push_error(errors, format!("{check_path}.help"), "must be a string");
        }
    }
}

fn validate_timeout(value: &Value, path: &str, errors: &mut Vec<ValidationIssue>) {
    let Some(timeout) = value.as_str() else {
        push_error(errors, path.to_string(), "must be a string");
        return;
    };

    if parse_timeout(timeout).is_err() {
        push_error(
            errors,
            path.to_string(),
            "must be a human-readable duration like `30 seconds` or `2 minutes`",
        );
    }
}

fn parse_timeout(timeout: &str) -> Result<Duration, ()> {
    let parts: Vec<_> = timeout.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(());
    }

    let value: u64 = parts[0].parse().map_err(|_| ())?;
    let seconds = match parts[1].to_ascii_lowercase().as_str() {
        "second" | "seconds" | "sec" | "secs" | "s" => value,
        "minute" | "minutes" | "min" | "mins" | "m" => value.checked_mul(60).ok_or(())?,
        _ => return Err(()),
    };

    Ok(Duration::from_secs(seconds))
}

pub fn validate(runbook: &Value) -> ValidationResult {
    let mut errors = Vec::new();
    let warnings = Vec::new();
    let mut global_datetime_anchor_ids = HashSet::new();
    let mut global_capture_names = HashSet::new();
    let mut available_capture_names = HashSet::new();

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
                    validate_entry(
                        entry,
                        index,
                        &mut errors,
                        &mut global_datetime_anchor_ids,
                        &mut global_capture_names,
                        &mut available_capture_names,
                    );
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
