//! Fixed-offset datetime rewriting and Chrono-compatible custom formats.
#[path = "datetime_format.rs"]
mod custom_format;
use super::{
    CommandExecution, RenderError, RewriteRuleResult, generated_rewrite_capture_from_pairs,
};
use custom_format::{format_custom, parse_custom};
use jiff::{civil::DateTime, tz::Offset};
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

const DEFAULT_DATETIME_SHIFT_BASE: &str = "2077-04-27T12:34:56.789+01:00";
const RFC3339_PATTERN: &str =
    r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,9})?(?:Z|[+-]\d{2}:\d{2})";
const RFC1123_PATTERN: &str = r"[A-Z][a-z]{2}, \d{2} [A-Z][a-z]{2} \d{4} \d{2}:\d{2}:\d{2} GMT";

#[derive(Clone, Default)]
pub(super) struct DatetimeRewriteState {
    pub(super) declarations: HashSet<String>,
    anchors: HashMap<String, DatetimeShiftAnchor>,
}

pub(super) fn apply_datetime_shift_rule(
    entry: &Value,
    execution: &CommandExecution,
    rule: &Value,
    rendered: &str,
    datetime_anchors: &mut DatetimeRewriteState,
) -> Result<RewriteRuleResult, RenderError> {
    let config = datetime_shift_config(rule)?;
    if let Some(anchor_id) = config.anchor_id
        && !datetime_anchors.declarations.insert(anchor_id.to_string())
    {
        return Err(RenderError::Operational(format!(
            "Duplicate datetime_shift id `{anchor_id}`: already declared by an executed rewrite rule"
        )));
    }
    let regex = Regex::new(config.pattern).map_err(|err| {
        RenderError::Operational(format!(
            "Invalid output rewrite pattern `{}`: {err}",
            config.pattern
        ))
    })?;
    let base_timestamp = parse_rfc3339(config.base).map_err(|err| {
        RenderError::Operational(format!(
            "Invalid datetime_shift base `{}`: {err}",
            config.base
        ))
    })?;
    let shared_anchor = match config.use_anchor {
        Some(anchor_id) => Some(datetime_anchors.anchors.get(anchor_id).ok_or_else(|| {
            RenderError::Operational(format!(
                "Command output datetime_shift uses unavailable anchor `{anchor_id}`"
            ))
        })?),
        None => None,
    };
    let parse_reference_timestamp = shared_anchor
        .map(|anchor| anchor.base_timestamp)
        .unwrap_or(base_timestamp);

    let mut first_original: Option<FixedDatetime> = None;
    let mut result = String::with_capacity(rendered.len());
    let mut last_end = 0;
    let mut capture_matches: Vec<(String, String)> = Vec::new();
    let match_count = regex.find_iter(rendered).count();

    for matched in regex.find_iter(rendered) {
        result.push_str(&rendered[last_end..matched.start()]);
        let matched_text = matched.as_str().to_string();
        let original =
            parse_shift_datetime(&matched_text, config.matcher, parse_reference_timestamp)?;
        let shifted = match shared_anchor {
            Some(anchor) => anchor.shift(original)?,
            None => match first_original {
                Some(first) => shift_from_anchor(base_timestamp, first, original)?,
                None => {
                    first_original = Some(original);
                    base_timestamp
                }
            },
        };
        let rewritten_value = format_shift_datetime(shifted, config.matcher, &matched_text)?;
        if rule.get("capture_as").is_some() {
            capture_matches.push((matched_text, rewritten_value.clone()));
        }
        result.push_str(&rewritten_value);
        last_end = matched.end();
    }

    result.push_str(&rendered[last_end..]);

    if let (Some(anchor_id), Some(first_original)) = (config.anchor_id, first_original) {
        datetime_anchors.anchors.insert(
            anchor_id.to_string(),
            DatetimeShiftAnchor {
                first_original,
                base_timestamp,
            },
        );
    }

    Ok(RewriteRuleResult {
        rendered: result,
        generated_capture: generated_rewrite_capture_from_pairs(
            entry,
            execution,
            rule,
            capture_matches,
        )?,
        debug_lines: vec![
            format!("[debug] Rewrite datetime_shift pattern: {}", config.pattern),
            format!("[debug] Rewrite datetime_shift match count: {match_count}"),
        ],
    })
}

struct DatetimeShiftConfig<'a> {
    pattern: &'a str,
    base: &'a str,
    matcher: DatetimeShiftMatcher<'a>,
    anchor_id: Option<&'a str>,
    use_anchor: Option<&'a str>,
}

#[derive(Clone, Copy)]
enum DatetimeShiftFormat {
    Rfc3339,
    Rfc1123,
}

#[derive(Clone, Copy)]
enum DatetimeShiftMatcher<'a> {
    BuiltIn(DatetimeShiftFormat),
    Custom(&'a str),
}

#[derive(Clone)]
struct DatetimeShiftAnchor {
    first_original: FixedDatetime,
    base_timestamp: FixedDatetime,
}

impl DatetimeShiftAnchor {
    fn shift(&self, original: FixedDatetime) -> Result<FixedDatetime, RenderError> {
        shift_from_anchor(self.base_timestamp, self.first_original, original)
    }
}

fn datetime_shift_config(rule: &Value) -> Result<DatetimeShiftConfig<'_>, RenderError> {
    let format = rule.get("format").and_then(Value::as_str);
    let pattern = rule.get("pattern").and_then(Value::as_str);
    let custom_format = rule.get("custom_format").and_then(Value::as_str);
    let base = rule
        .get("base")
        .and_then(Value::as_str)
        .unwrap_or(DEFAULT_DATETIME_SHIFT_BASE);
    let anchor_id = rule.get("id").and_then(Value::as_str);
    let use_anchor = rule.get("use").and_then(Value::as_str);

    if format.is_some() && pattern.is_some() {
        return Err(RenderError::Operational(
            "Command output datetime_shift must not include both format and pattern".to_string(),
        ));
    }

    if format.is_some() && custom_format.is_some() {
        return Err(RenderError::Operational(
            "Command output datetime_shift must not include both format and custom_format"
                .to_string(),
        ));
    }

    if anchor_id.is_some() && use_anchor.is_some() {
        return Err(RenderError::Operational(
            "Command output datetime_shift must not include both id and use".to_string(),
        ));
    }

    if use_anchor.is_some() && rule.get("base").is_some() {
        return Err(RenderError::Operational(
            "Command output datetime_shift that uses an anchor must not include base".to_string(),
        ));
    }

    let (pattern, matcher) = match (format, pattern, custom_format) {
        (Some("rfc3339"), None, None) => (
            RFC3339_PATTERN,
            DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc3339),
        ),
        (Some("rfc1123"), None, None) => (
            RFC1123_PATTERN,
            DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc1123),
        ),
        (Some(other), None, None) => {
            return Err(RenderError::Operational(format!(
                "Unsupported datetime_shift format `{other}`"
            )));
        }
        (None, Some(pattern), Some(custom_format)) => {
            (pattern, DatetimeShiftMatcher::Custom(custom_format))
        }
        (None, Some(_), None) => {
            return Err(RenderError::Operational(
                "Command output datetime_shift with pattern must include custom_format".to_string(),
            ));
        }
        (None, None, Some(_)) => {
            return Err(RenderError::Operational(
                "Command output datetime_shift custom_format requires pattern".to_string(),
            ));
        }
        (None, None, None) => {
            return Err(RenderError::Operational(
                "Command output datetime_shift must include either format or pattern".to_string(),
            ));
        }
        _ => unreachable!(),
    };

    Ok(DatetimeShiftConfig {
        pattern,
        base,
        matcher,
        anchor_id,
        use_anchor,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FixedDatetime {
    local: DateTime,
    offset: Offset,
}

fn operational(error: impl std::fmt::Display) -> RenderError {
    RenderError::Operational(error.to_string())
}

fn parse_rfc3339(value: &str) -> Result<FixedDatetime, String> {
    // The Temporal parser accepts extensions and clamps leap seconds. Keep the
    // RFC 3339 boundary explicit instead of silently accepting those changes.
    static SHAPE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}[Tt ][0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:[Zz]|[+-][0-9]{2}:[0-9]{2})$")
        .expect("constant RFC 3339 expression")
    });
    if !SHAPE.is_match(value) {
        return Err("invalid RFC 3339 syntax".into());
    }
    let mut value = value.to_string();
    value.replace_range(10..11, "T");
    let value = if value.ends_with(['Z', 'z']) {
        format!("{}+00:00", &value[..value.len() - 1])
    } else {
        value
    };
    let parsed = parse_custom(&value, "%Y-%m-%dT%H:%M:%S%.f%:z")?;
    let offset = parsed.offset().ok_or("missing RFC 3339 offset")?;
    let local = parsed.to_datetime().map_err(|e| e.to_string())?;
    offset.to_timestamp(local).map_err(|e| e.to_string())?;
    Ok(FixedDatetime { local, offset })
}

fn parse_shift_datetime(
    value: &str,
    matcher: DatetimeShiftMatcher<'_>,
    base: FixedDatetime,
) -> Result<FixedDatetime, RenderError> {
    match matcher {
        DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc3339) => parse_rfc3339(value)
            .map_err(|err| {
                operational(format!(
                    "Matched datetime `{value}` is not valid RFC 3339: {err}"
                ))
            }),
        DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc1123) => {
            // The regex limits RFC 2822 input to this GMT shape. Conversion
            // validates the weekday; do not apply the custom time-only fallback.
            parse_custom(value, "%a, %d %b %Y %H:%M:%S GMT")
                .and_then(|tm| tm.to_datetime().map_err(|e| e.to_string()))
                .map(|local| FixedDatetime {
                    local,
                    offset: Offset::UTC,
                })
                .map_err(|err| {
                    operational(format!(
                        "Matched datetime `{value}` is not valid RFC 1123: {err}"
                    ))
                })
        }
        DatetimeShiftMatcher::Custom(format) => {
            parse_local_datetime(value, format, base).map_err(|err| {
                operational(format!(
                    "Matched datetime `{value}` does not match custom_format `{format}`: {err}"
                ))
            })
        }
    }
}

fn parse_local_datetime(
    value: &str,
    format: &str,
    base: FixedDatetime,
) -> Result<FixedDatetime, String> {
    let parsed = parse_custom(value, format)?;
    let local = parsed.to_local_datetime(base.local)?;
    base.offset.to_timestamp(local).map_err(|e| e.to_string())?;
    Ok(FixedDatetime {
        local,
        offset: base.offset,
    })
}

fn format_shift_datetime(
    datetime: FixedDatetime,
    matcher: DatetimeShiftMatcher<'_>,
    original: &str,
) -> Result<String, RenderError> {
    match matcher {
        DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc3339) => {
            let zulu = original.ends_with('Z');
            let output = if zulu {
                let timestamp = datetime
                    .offset
                    .to_timestamp(datetime.local)
                    .map_err(operational)?;
                FixedDatetime {
                    local: Offset::UTC.to_datetime(timestamp),
                    offset: Offset::UTC,
                }
            } else {
                datetime
            };
            let width = original
                .split_once('.')
                .map(|(_, rest)| rest.bytes().take_while(u8::is_ascii_digit).count());
            let mut result = format_custom(output, "%Y-%m-%dT%H:%M:%S")?;
            if let Some(width) = width {
                result.push('.');
                result.push_str(&format!("{:09}", output.local.subsec_nanosecond())[..width]);
            }
            result.push_str(&if zulu {
                "Z".to_string()
            } else {
                format_custom(output, "%:z")?
            });
            Ok(result)
        }
        DatetimeShiftMatcher::BuiltIn(DatetimeShiftFormat::Rfc1123) => {
            format_custom(datetime, "%a, %d %b %Y %H:%M:%S GMT")
        }
        DatetimeShiftMatcher::Custom(format) => format_custom(datetime, format),
    }
}

fn shift_from_anchor(
    base: FixedDatetime,
    first: FixedDatetime,
    original: FixedDatetime,
) -> Result<FixedDatetime, RenderError> {
    let first = first
        .offset
        .to_timestamp(first.local)
        .map_err(operational)?;
    let original = original
        .offset
        .to_timestamp(original.local)
        .map_err(operational)?;
    let delta = original.duration_since(first);
    let local = base
        .local
        .checked_add(delta)
        .map_err(|err| operational(format!("datetime_shift result is out of range: {err}")))?;
    base.offset.to_timestamp(local).map_err(operational)?;
    Ok(FixedDatetime {
        local,
        offset: base.offset,
    })
}

#[cfg(test)]
mod datetime_anchor_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn duplicate_declaration_preserves_the_established_anchor() {
        let execution = CommandExecution {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            timed_out: false,
            timeout_cleanup_failure: None,
        };
        let entry = json!({"type": "Command", "commands": ["echo timestamp"]});
        let rule = json!({"type": "datetime_shift", "id": "timeline", "format": "rfc3339"});
        let mut state = DatetimeRewriteState::default();
        apply_datetime_shift_rule(
            &entry,
            &execution,
            &rule,
            "2026-03-12T12:00:00Z",
            &mut state,
        )
        .unwrap_or_else(|_| panic!("expected successful datetime rewrite"));
        let first_anchor = state.anchors["timeline"].clone();
        for text in ["2020-01-01T00:00:00Z", "no timestamps"] {
            let mut duplicate = rule.clone();
            duplicate["base"] = json!("2099-01-01T00:00:00Z");
            assert!(matches!(
                apply_datetime_shift_rule(&entry, &execution, &duplicate, text, &mut state),
                Err(RenderError::Operational(_))
            ));
            assert_eq!(
                state.anchors["timeline"].first_original,
                first_anchor.first_original
            );
            assert_eq!(
                state.anchors["timeline"].base_timestamp,
                first_anchor.base_timestamp
            );
        }
        let use_rule = json!({"type": "datetime_shift", "use": "timeline", "format": "rfc3339"});
        let result = apply_datetime_shift_rule(
            &entry,
            &execution,
            &use_rule,
            "2026-03-12T12:00:02Z",
            &mut state,
        )
        .unwrap_or_else(|_| panic!("expected successful datetime rewrite"));
        assert_eq!(result.rendered, "2077-04-27T11:34:58Z");
    }
}
