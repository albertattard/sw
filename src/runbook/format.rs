use serde_json::{Map, Value};

const COMMAND_FIELD_ORDER: &[&str] = &[
    "type",
    "debug",
    "execute_when",
    "working_directory",
    "working_dir",
    "environment",
    "commands",
    "assert",
    "output",
    "capture",
    "cleanup",
    "timeout",
    "indent",
];

pub(crate) fn order_schema_fields(document: &mut Value) {
    order_fields(document, &["secrets", "entries"]);
    for_map_values(document.get_mut("secrets"), |declaration| {
        order_fields(declaration, &["generate"]);
        if let Some(generator) = declaration.get_mut("generate") {
            order_fields(generator, &["length"]);
        }
    });
    let Some(entries) = document.get_mut("entries").and_then(Value::as_array_mut) else {
        return;
    };

    for entry in entries {
        order_nested_entry_fields(entry);
        let Some(fields) = entry.as_object_mut() else {
            continue;
        };
        let field_order: &[&str] = match fields.get("type").and_then(Value::as_str) {
            Some("Command") => COMMAND_FIELD_ORDER,
            Some("Heading") => &["type", "level", "title"],
            Some("Markdown") => &["type", "contents", "indent"],
            Some("ChangeDirectory") => &["type", "path", "contents"],
            Some("DisplayFile") => &[
                "type",
                "path",
                "content_type",
                "start_line",
                "line_count",
                "transform",
                "show_trim_markers",
                "offset",
                "indent",
            ],
            Some("DisplayUrl") => &[
                "type",
                "url",
                "timeout",
                "start_line",
                "line_count",
                "content_type",
                "show_trim_markers",
                "offset",
                "indent",
            ],
            Some("Patch") => &["type", "path", "patch", "restore", "indent", "debug"],
            Some("Prerequisite") => &["type", "checks"],
            Some("Breakpoint") => &["type", "message"],
            _ => continue,
        };

        reorder_map(fields, field_order);
    }
}

// Traverse only schema paths: accepted extra fields and user-defined map keys
// must never be treated as schema objects merely because their names match.
fn order_nested_entry_fields(entry: &mut Value) {
    match entry.get("type").and_then(Value::as_str) {
        Some("Command") => {
            if let Some(condition) = entry.get_mut("execute_when") {
                order_fields(condition, &["fact", "equals"]);
            }
            for_map_values(entry.get_mut("environment"), |binding| {
                order_fields(binding, &["secret"]);
            });
            if let Some(assertion) = entry.get_mut("assert") {
                order_assertion(assertion);
            }
            if let Some(output) = entry.get_mut("output") {
                order_fields(
                    output,
                    &[
                        "caption",
                        "stream",
                        "content_type",
                        "trim_empty_lines",
                        "trim_trailing_whitespace",
                        "rewrite",
                    ],
                );
                for_array_items(output.get_mut("rewrite"), order_rewrite);
            }
            for_array_items(entry.get_mut("capture"), |capture| {
                order_fields(capture, &["name", "source", "stage", "pattern", "parse_as"]);
                if let Some(parse_as) = capture.get_mut("parse_as") {
                    order_fields(
                        parse_as,
                        &["type", "locale", "decimal_separator", "grouping_separator"],
                    );
                }
            });
        }
        Some("Prerequisite") => for_array_items(entry.get_mut("checks"), |check| {
            let fields: &[&str] = match check.get("kind").and_then(Value::as_str) {
                Some("java") => &[
                    "kind",
                    "name",
                    "version",
                    "distribution",
                    "java_home",
                    "java_home_env",
                    "contents",
                    "help",
                ],
                Some("command") => &[
                    "kind", "name", "commands", "assert", "timeout", "contents", "help",
                ],
                _ => return,
            };
            order_fields(check, fields);
            if let Some(assertion) = check.get_mut("assert") {
                order_assertion(assertion);
            }
        }),
        Some("DisplayFile") => {
            if let Some(transform) = entry.get_mut("transform") {
                order_fields(transform, &["language", "operations"]);
                for_array_items(transform.get_mut("operations"), |operation| {
                    order_fields(operation, &["type", "name", "replacement"]);
                });
            }
        }
        _ => {}
    }
}

fn order_assertion(assertion: &mut Value) {
    order_fields(assertion, &["exit_code", "checks"]);
    for_array_items(assertion.get_mut("checks"), |check| {
        order_fields(check, &["source", "path", "exists", "sha256", "contains"]);
    });
}

fn order_rewrite(rewrite: &mut Value) {
    let fields: &[&str] = match rewrite.get("type").and_then(Value::as_str) {
        Some("replace") => &["type", "pattern", "replacement", "capture_as"],
        Some("keep_between") => &[
            "type",
            "start",
            "start_pattern",
            "end",
            "end_pattern",
            "start_offset",
            "end_offset",
            "show_trim_markers",
        ],
        Some("limit_lines") => &["type", "first", "last", "show_trim_marker"],
        Some("datetime_shift") => &[
            "type",
            "pattern",
            "base",
            "format",
            "id",
            "use",
            "custom_format",
            "capture_as",
        ],
        _ => return,
    };
    order_fields(rewrite, fields);
}

fn for_map_values(value: Option<&mut Value>, mut visit: impl FnMut(&mut Value)) {
    if let Some(fields) = value.and_then(Value::as_object_mut) {
        for value in fields.values_mut() {
            visit(value);
        }
    }
}

fn for_array_items(value: Option<&mut Value>, mut visit: impl FnMut(&mut Value)) {
    if let Some(items) = value.and_then(Value::as_array_mut) {
        for item in items {
            visit(item);
        }
    }
}

fn order_fields(value: &mut Value, field_order: &[&str]) {
    if let Some(fields) = value.as_object_mut() {
        reorder_map(fields, field_order);
    }
}

fn reorder_map(fields: &mut Map<String, Value>, field_order: &[&str]) {
    let mut remaining = std::mem::take(fields);
    let mut ordered = Map::with_capacity(remaining.len());
    for &key in field_order {
        if let Some(value) = remaining.shift_remove(key) {
            ordered.insert(key.to_string(), value);
        }
    }
    // Retain accepted unlisted fields in their original relative order.
    ordered.extend(remaining);
    *fields = ordered;
}
