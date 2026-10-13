use serde_json::{Map, Value};

const COMMAND_FIELD_ORDER: &[&str] = &[
    "type",
    "debug",
    "execute_when",
    "working_directory",
    "working_dir",
    "commands",
    "assert",
    "output",
    "capture",
    "cleanup",
    "timeout",
    "indent",
];

pub(crate) fn order_entry_fields(document: &mut Value) {
    let Some(entries) = document.get_mut("entries").and_then(Value::as_array_mut) else {
        return;
    };

    for entry in entries {
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

        let mut remaining = std::mem::take(fields);
        let mut ordered = Map::with_capacity(remaining.len());
        for &key in field_order {
            if let Some(value) = remaining.shift_remove(key) {
                ordered.insert(key.to_string(), value);
            }
        }
        // Keep any future fields rather than silently dropping their values.
        ordered.extend(remaining);
        *fields = ordered;
    }
}
