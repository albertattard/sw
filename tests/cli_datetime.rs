//! Compatibility regressions for the datetime dependency migration (TASK-208).
use serde_json::{Value, json};
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

fn rewrite(text: &str, mut rule: Value) -> (std::process::Output, String) {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "sw-datetime-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).unwrap();
    rule["type"] = json!("datetime_shift");
    let document = json!({"entries": [{"type": "Command",
        "commands": [format!("printf '%s\\n' '{}'", text.replace('\'', "'\\''"))],
        "output": {"rewrite": [rule]}}]});
    fs::write(dir.join("sw-runbook.json"), document.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_sw"))
        .arg("run")
        .current_dir(&dir)
        .output()
        .unwrap();
    let markdown = fs::read_to_string(dir.join("README.md")).unwrap_or_default();
    fs::remove_dir_all(dir).unwrap();
    (output, markdown)
}

fn assert_rewrite(text: &str, rule: Value, expected: &str) {
    let (output, markdown) = rewrite(text, rule);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        markdown.contains(expected),
        "expected {expected:?} in {markdown}"
    );
}

#[test]
fn rfc3339_all_fractional_widths_and_offset_styles() {
    for width in 1..=9 {
        for suffix in ["Z", "+05:45", "-03:30"] {
            let fraction = &"123456789"[..width];
            let expected_fraction = &"987654321"[..width];
            let output_suffix = if suffix == "Z" { "Z" } else { "+02:30" };
            let clock = if suffix == "Z" { "21:00" } else { "23:30" };
            assert_rewrite(
                &format!("2026-03-12T12:00:00.{fraction}{suffix}"),
                json!({"format":"rfc3339", "base":"2077-04-27T23:30:00.987654321+02:30"}),
                &format!("2077-04-27T{clock}:00.{expected_fraction}{output_suffix}"),
            );
        }
    }
}

#[test]
fn custom_fraction_semantics_and_composite_formats() {
    for (format, input, expected) in [
        ("%H:%M:%S %f", "12:34:56 123", "23:59:58 123400000"),
        ("%H:%M:%S %3f", "12:34:56 123", "23:59:58 123"),
        ("%H:%M:%S %6f", "12:34:56 123456", "23:59:58 123400"),
        ("%H:%M:%S %9f", "12:34:56 123456789", "23:59:58 123400000"),
        ("%H:%M:%S%.f", "12:34:56.1", "23:59:58.123400"),
        ("%H:%M:%S%.3f", "12:34:56.123", "23:59:58.123"),
        (
            "%+",
            "2026-03-12T12:00:00+05:45",
            "2077-04-27T23:59:58.123400+02:30",
        ),
        ("%c", "Thu Mar 12 12:00:00 2026", "Tue Apr 27 23:59:58 2077"),
        ("%x %X", "03/12/26 12:00:00", "04/27/77 23:59:58"),
        ("%r", "12:34:56 PM", "11:59:58 PM"),
        ("%s", "0", "3386784598"),
        (
            "%Y-%m-%d %H:%M:%S %:z",
            "2026-03-12 12:00:00 -03:30",
            "2077-04-27 23:59:58 +02:30",
        ),
    ] {
        assert_rewrite(
            input,
            json!({"pattern":"[^\\n]+", "custom_format":format,
            "base":"2077-04-27T23:59:58.123400+02:30"}),
            expected,
        );
    }
}

#[test]
fn custom_offsets_are_validated_but_use_the_base_offset_for_elapsed_time() {
    assert_rewrite(
        "2026-03-12 12:00:00 +0545\n2026-03-12 12:00:01 -0330",
        json!({"pattern":"[^\\n]+", "custom_format":"%Y-%m-%d %H:%M:%S %z",
            "base":"2077-04-27T23:59:59+02:30"}),
        "2077-04-27 23:59:59 +0230\n2077-04-28 00:00:00 +0230",
    );
}

#[test]
fn custom_offsets_accept_chrono_relaxed_utc_and_numeric_syntax() {
    for suffix in ["UTC", "utc"] {
        assert_custom_output(
            &format!("2026-03-12T12:00:00{suffix}"),
            "%+",
            "2077-04-27T12:00:00+00:00",
        );
    }
    for suffix in [" +01:30", "+01 30", "−01:30"] {
        assert_custom_output(&format!("12:00:00{suffix}"), "%T%z", "12:00:00+0000");
    }
}

#[test]
fn rfc1123_retains_local_base_clock_and_literal_gmt() {
    assert_rewrite(
        "Thu, 12 Mar 2026 12:00:00 GMT\nThu, 12 Mar 2026 12:00:02 GMT",
        json!({"format":"rfc1123", "base":"2077-04-27T23:59:59+05:45"}),
        "Tue, 27 Apr 2077 23:59:59 GMT\nWed, 28 Apr 2077 00:00:01 GMT",
    );
}

#[test]
fn invalid_dates_weekdays_offsets_and_fraction_widths_fail_operationally() {
    for (text, rule) in [
        ("2026-02-30T12:00:00Z", json!({"format":"rfc3339"})),
        ("2026-03-12T12:00:00+24:00", json!({"format":"rfc3339"})),
        ("Fri, 12 Mar 2026 12:00:00 GMT", json!({"format":"rfc1123"})),
        (
            "12:00:00.1",
            json!({"pattern":"[^\\n]+", "custom_format":"%H:%M:%S%.3f"}),
        ),
        (
            "12:00:00 123",
            json!({"pattern":"[^\\n]+", "custom_format":"%H:%M:%S %9f"}),
        ),
        (
            "12:00:00",
            json!({"pattern":"[^\\n]+", "custom_format":"%Q"}),
        ),
        (
            "2026-03-12T12:00:00Z",
            json!({"format":"rfc3339", "base":"invalid"}),
        ),
    ] {
        let (output, _) = rewrite(text, rule);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{text}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("datetime_shift"), "{stderr}");
        assert!(!stderr.contains("panicked"), "{stderr}");
    }
}

#[test]
fn raw_nanoseconds_and_decimal_fractions_preserve_elapsed_distances() {
    for (format, text, expected) in [
        (
            "%H:%M:%S %f",
            "12:34:56 123\n12:34:56 124",
            "00:00:00 000000001",
        ),
        (
            "%H:%M:%S%.f",
            "12:34:56.1\n12:34:56.1234",
            "00:00:00.023400",
        ),
        ("%H:%M:%S %3f", "12:34:56 123\n12:34:56 124", "00:00:00 001"),
    ] {
        assert_rewrite(
            text,
            json!({"pattern":"[^\\n]+", "custom_format": format,
            "base":"2077-04-27T00:00:00Z"}),
            expected,
        );
    }
}

#[test]
fn rfc3339_different_offsets_and_negative_distances_use_actual_instants() {
    assert_rewrite(
        "2026-03-12T12:00:00+05:45\n2026-03-12T02:44:59.999999999-03:30",
        json!({"format":"rfc3339", "base":"2077-04-27T00:00:00.000000001+02:30"}),
        "2077-04-27T00:00:00.000000000+02:30",
    );
}

#[test]
fn leap_seconds_and_range_overflow_fail_without_panicking() {
    for (text, rule) in [
        ("2016-12-31T23:59:60Z", json!({"format":"rfc3339"})),
        ("Sat, 31 Dec 2016 23:59:60 GMT", json!({"format":"rfc1123"})),
        (
            "23:59:60",
            json!({"pattern":"[^\\n]+", "custom_format":"%T"}),
        ),
        (
            "-10000-01-01 00:00:00",
            json!({"pattern":"[^\\n]+", "custom_format":"%Y-%m-%d %T"}),
        ),
        (
            "2026-03-12T12:00:00Z",
            json!({"format":"rfc3339", "base":"2016-12-31T23:59:60Z"}),
        ),
        (
            "2026-03-12T12:00:00Z\n2026-03-12T12:00:01Z",
            json!({"format":"rfc3339", "base":"9999-12-31T23:59:59Z"}),
        ),
        (
            "9999-12-31T23:59:59Z\n0000-01-01T00:00:00Z",
            json!({"format":"rfc3339", "base":"0000-01-01T00:00:00Z"}),
        ),
    ] {
        let (output, _) = rewrite(text, rule);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{text}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("datetime_shift"), "{stderr}");
        assert!(stderr.contains("output.rewrite[0]"), "{stderr}");
        assert!(!stderr.contains("panicked"), "{stderr}");
    }
}

#[test]
fn custom_directives_keep_posix_padding_week_dates_epoch_and_zone_behavior() {
    for (format, text, expected) in [
        ("%H:%M", "12:34", "23:59"),
        ("%v %T", "12-Mar-2026 12:34:56", "27-Apr-2077 23:59:58"),
        ("%Y-%j %H:%M", "2026-071 12:34", "2077-117 23:59"),
        ("%G-W%V-%u %H:%M", "2026-W11-4 12:34", "2077-W17-2 23:59"),
        ("%C%y-%m-%d %H:%M", "2026-03-12 12:34", "2077-04-27 23:59"),
        (
            "%Y-%q-%m-%d %H:%M",
            "2026-1-03-12 12:34",
            "2077-2-04-27 23:59",
        ),
        ("%H:%M:%S %_f", "12:34:56     12345", "23:59:58 123400000"),
        ("%H:%M:%S %-f", "12:34:56 12345", "23:59:58 123400000"),
        ("%H:%M:%S %0f", "12:34:56 12345", "23:59:58 123400000"),
        ("%H:%M:%S%.f", "12:34:56.123456789123", "23:59:58.123400"),
        (
            "%Y-%m-%d %H:%M:%S %Z",
            "2026-03-12 12:34:56 UTC",
            "2077-04-27 23:59:58 +02:30",
        ),
        ("%s %z", "0 +0130", "3386784598 +0230"),
        (
            "%+",
            "2026-03-12 12:34:56Z",
            "2077-04-27T23:59:58.123400+02:30",
        ),
    ] {
        assert_rewrite(
            text,
            json!({"pattern":"[^\\n]+", "custom_format":format,
            "base":"2077-04-27T23:59:58.123400+02:30"}),
            expected,
        );
    }
}

#[test]
fn custom_datetime_preserves_historical_time_only_fallback_for_inconsistent_dates() {
    // Chrono tried NaiveDateTime, then NaiveTime with the very same format.
    for (format, text) in [
        ("%Y-%m-%d %T", "2026-02-30 12:34:56"),
        ("%Y-%m-%d %a %T", "2026-03-12 Fri 12:34:56"),
    ] {
        assert_rewrite(
            text,
            json!({"pattern":"[^\\n]+", "custom_format":format}),
            "2077",
        );
    }
    for (format, text) in [
        ("%H", "12"),
        ("%Y-%m-%d %H:%M:%S %_a", "2026-03-12 12:34:56 Thu"),
        ("%Y-%m-%d", "2026-03-12"),
        ("%H:%M:%S %z", "12:34:56 Z"),
        ("%H:%M:%S %::z", "12:34:56 +01:30:00"),
        ("%H:%M:%S %:::z", "12:34:56 +01"),
    ] {
        let (output, _) = rewrite(text, json!({"pattern":"[^\\n]+", "custom_format":format}));
        assert_eq!(output.status.code(), Some(1));
    }
}

#[test]
fn epoch_fractions_and_repeated_fields_preserve_parsing_contract() {
    assert_rewrite(
        "0.100000000\n0.100000001",
        json!({
            "pattern":"[^\\n]+", "custom_format":"%s%.f", "base":"2077-04-27T00:00:00Z"
        }),
        "3386707200.000000001",
    );
    for (format, text) in [
        ("%H:%M:%S %H", "12:34:56 13"),
        ("%H:%M:%S %f %f", "12:34:56 1 2"),
        ("%s %f", "-1 123"),
    ] {
        let (output, _) = rewrite(text, json!({"pattern":"[^\\n]+", "custom_format":format}));
        assert_eq!(output.status.code(), Some(1));
    }
}

#[test]
fn rfc3339_base_retains_strict_fields_and_case_insensitive_separator() {
    assert_rewrite(
        "2026-03-12T12:00:00Z",
        json!({"format":"rfc3339",
        "base":"2077-04-27t00:00:00.123456789123z"}),
        "2077-04-27T00:00:00Z",
    );
    for base in [
        "2077-4-27T00:00:00Z",
        "2077-04-27T00:00:00+0230",
        "2077-04-27T00:00:00.Z",
    ] {
        let (output, _) = rewrite(
            "2026-03-12T12:00:00Z",
            json!({"format":"rfc3339", "base":base}),
        );
        assert_eq!(output.status.code(), Some(1));
    }
}

#[test]
fn century_format_retains_zero_padding() {
    assert_rewrite(
        "2026-03-12 12:34:56",
        json!({"pattern":"[^\\n]+",
        "custom_format":"%C%y-%m-%d %T", "base":"0001-04-27T12:34:56Z"}),
        "0001-04-27 12:34:56",
    );
}

fn assert_custom_output(text: &str, format: &str, expected: &str) {
    assert_custom_output_at(text, format, "2077-04-27T12:00:00Z", expected);
}

fn assert_custom_output_at(text: &str, format: &str, base: &str, expected: &str) {
    let (output, markdown) = rewrite(
        text,
        json!({
            "pattern": "[^|\\n]+", "custom_format": format, "base": base
        }),
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{format}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        markdown.ends_with(&format!("```\n{expected}\n```\n")),
        "{format}: expected {expected:?}, got {markdown:?}"
    );
}

#[test]
fn custom_year_components_preserve_leap_day_distances_in_either_order() {
    for (format, year1900, year2000, shifted_year) in [
        ("%C%y", "1900", "2000", "2077"),
        ("%y%C", "0019", "0020", "7720"),
        ("%Y %y", "1900 00", "2000 00", "2077 77"),
        ("%y %Y", "00 1900", "00 2000", "77 2077"),
        ("%Y %C %y", "1900 19 00", "2000 20 00", "2077 20 77"),
        ("%y %C %Y", "00 19 1900", "00 20 2000", "77 20 2077"),
    ] {
        let format = format!("{format}-%m-%d %T");
        for (year, last_day) in [(year1900, "28"), (year2000, "29")] {
            assert_custom_output(
                &format!("{year}-02-28 12:00:00|{year}-03-01 12:00:00"),
                &format,
                &format!("{shifted_year}-04-27 12:00:00|{shifted_year}-04-{last_day} 12:00:00"),
            );
        }
    }
    assert_custom_output(
        "1899-12-31 12:00:00|1900-01-01 12:00:00",
        "%C%y-%m-%d %T",
        "2077-04-27 12:00:00|2077-04-28 12:00:00",
    );
}

#[test]
fn custom_year_components_keep_conflict_fallback_and_the_chrono_pivot() {
    for (format, year, shifted) in [
        ("%Y %y", "1900 01", "2077 77"),
        ("%y %Y", "01 1900", "77 2077"),
        ("%Y %C", "1900 20", "2077 20"),
        ("%C %Y", "20 1900", "20 2077"),
    ] {
        assert_custom_output(
            &format!("{year}-02-28 12:00:00|{year}-03-01 12:00:00"),
            &format!("{format}-%m-%d %T"),
            &format!("{shifted}-04-27 12:00:00|{shifted}-04-27 12:00:00"),
        );
    }
    for (format, text) in [
        ("%C %C%y-%m-%d %T", "19 2000-02-28 12:00:00"),
        ("%y %y-%m-%d %T", "00 01-02-28 12:00:00"),
        ("%Y %Y-%m-%d %T", "1900 2000-02-28 12:00:00"),
    ] {
        let (output, _) = rewrite(text, json!({"pattern":"[^|\\n]+", "custom_format": format}));
        assert_eq!(
            output.status.code(),
            Some(1),
            "{format}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
    // Chrono maps 69 to 2069 and 70 to 1970. Jiff's native pivot is 68/69.
    assert_custom_output_at(
        "68-12-31 12:00:00|69-01-01 12:00:00",
        "%y-%m-%d %T",
        "2000-02-28T12:00:00Z",
        "00-02-28 12:00:00|00-02-29 12:00:00",
    );
}

#[test]
fn custom_literal_letters_are_not_datetime_directives() {
    for prefix in ["logs_", "s", "f", "z", "S", "%_"] {
        let format = format!("{}%Y%m%d_%H%M%S", prefix.replace('%', "%%"));
        assert_custom_output(
            &format!("{prefix}20260312_123456|{prefix}20260312_123457"),
            &format,
            &format!("{prefix}20770427_120000|{prefix}20770427_120001"),
        );
    }
    for format in ["%s", "%_s", "%-s", "%0s"] {
        for input in ["0|1", " 0|\t1"] {
            assert_custom_output(input, format, "3386750400|3386750401");
        }
    }
}

#[test]
fn custom_full_month_directive_accepts_abbreviations_and_formats_full_names() {
    for month in ["Mar", "March", "mar", "mArCh", "MAR"] {
        assert_custom_output(
            &format!("2026-{month}-12 12:34:56|2026-Apr-12 12:34:56"),
            "%Y-%B-%d %T",
            "2077-April-27 12:00:00|2077-May-28 12:00:00",
        );
    }
    let (output, _) = rewrite(
        "2026-Mzz-12 12:34:56",
        json!({
            "pattern":"[^|\\n]+", "custom_format":"%Y-%B-%d %T"
        }),
    );
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn custom_whitespace_directives_match_zero_or_more_characters() {
    for (directive, printed) in [("%n", "\n"), ("%t", "\t")] {
        for whitespace in ["", " ", "\t", " \t "] {
            assert_custom_output(
                &format!("12:34:56{whitespace}|12:34:57{whitespace}"),
                &format!("%T{directive}"),
                &format!("12:00:00{printed}|12:00:01"),
            );
            assert_custom_output(
                &format!("12:34:56{whitespace}end|12:34:57{whitespace}end"),
                &format!("%T{directive}end"),
                &format!("12:00:00{printed}end|12:00:01{printed}end"),
            );
        }
    }
}

#[test]
fn custom_full_weekday_directive_accepts_abbreviations_without_changing_output_style() {
    for (first, second) in [("Thu", "Fri"), ("Thursday", "Friday"), ("thu", "FRIDAY")] {
        assert_custom_output(
            &format!("2026-03-12 {first} 12:00:00|2026-03-13 {second} 12:00:00"),
            "%Y-%m-%d %A %T",
            "2077-04-27 Tuesday 12:00:00|2077-04-28 Wednesday 12:00:00",
        );
    }
}

#[test]
fn custom_iso_year_components_preserve_century_and_pivot_semantics() {
    for (format, text, expected) in [
        (
            "%G %g-W%V-%u %T",
            "1900 00-W09-3 12:00:00|1900 00-W09-4 12:00:00",
            "2077 77-W17-2 12:00:00|2077 77-W17-3 12:00:00",
        ),
        (
            "%g %G-W%V-%u %T",
            "00 1900-W09-3 12:00:00|00 1900-W09-4 12:00:00",
            "77 2077-W17-2 12:00:00|77 2077-W17-3 12:00:00",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
    assert_custom_output_at(
        "68-W52-7 12:00:00|69-W01-1 12:00:00",
        "%g-W%V-%u %T",
        "2000-02-28T12:00:00Z",
        "00-W09-1 12:00:00|00-W09-2 12:00:00",
    );
}

#[test]
fn custom_epoch_calendar_conflicts_preserve_time_only_fallback_in_either_order() {
    for (format, text, expected) in [
        (
            "%s %Y-%m-%d %T",
            "0 2026-03-12 12:00:00|86400 2026-03-12 13:00:00",
            "3386750400 2077-04-27 12:00:00|3386754000 2077-04-27 13:00:00",
        ),
        (
            "%Y-%m-%d %T %s",
            "2026-03-12 12:00:00 0|2026-03-12 13:00:00 86400",
            "2077-04-27 12:00:00 3386750400|2077-04-27 13:00:00 3386754000",
        ),
        (
            "%s %T",
            "0 12:00:00|86400 13:00:00",
            "3386750400 12:00:00|3386754000 13:00:00",
        ),
        (
            "%s %Y-%m-%d %T",
            "0 1970-01-01 12:00:00|86400 1970-01-02 13:00:00",
            "3386750400 2077-04-27 12:00:00|3386754000 2077-04-27 13:00:00",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
}

#[test]
fn custom_epoch_consistent_and_partial_fields_retain_the_epoch_date() {
    for (format, text, expected) in [
        (
            "%s %Y-%m-%d %T",
            "0 1970-01-01 00:00:00|90000 1970-01-02 01:00:00",
            "3386750400 2077-04-27 12:00:00|3386840400 2077-04-28 13:00:00",
        ),
        (
            "%Y-%m-%d %T %s",
            "1970-01-01 00:00:00 0|1970-01-02 01:00:00 90000",
            "2077-04-27 12:00:00 3386750400|2077-04-28 13:00:00 3386840400",
        ),
        (
            "%s %Y %H",
            "0 1970 00|90000 1970 01",
            "3386750400 2077 12|3386840400 2077 13",
        ),
        ("%s %I", "0 12|90000 01", "3386750400 12|3386840400 01"),
        ("%s %p", "0 AM|90000 AM", "3386750400 PM|3386840400 PM"),
    ] {
        assert_custom_output(text, format, expected);
    }
}

#[test]
fn custom_epoch_conflicts_with_incomplete_times_fail_operationally() {
    for (format, text) in [
        ("%s %Y %H", "0 2026 12"),
        ("%s %S", "0 01"),
        ("%s %I", "0 01"),
        ("%s %p", "0 PM"),
    ] {
        let (output, _) = rewrite(text, json!({"pattern":"[^|\\n]+", "custom_format":format}));
        assert_eq!(
            output.status.code(),
            Some(1),
            "{format}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("output.rewrite[0]"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn custom_hour_meridiem_contradictions_fail_in_every_directive_order() {
    for (format, text) in [
        ("%H:%M:%S %p", "12:34:56 AM"),
        ("%p %H:%M:%S", "AM 12:34:56"),
        ("%I %p (%H:%M:%S)", "01 PM (12:34:56)"),
        ("%H:%M:%S (%I %p)", "12:34:56 (01 PM)"),
        ("%p %I (%H:%M:%S)", "PM 01 (12:34:56)"),
        ("%H:%M:%S %P", "00:34:56 pm"),
        ("%I:%M:%S", "01:34:56"),
        ("%s %H:%M:%S %p", "0 12:34:56 AM"),
    ] {
        let (output, _) = rewrite(text, json!({"pattern":"[^|\\n]+", "custom_format":format}));
        assert_eq!(
            output.status.code(),
            Some(1),
            "{format}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("output.rewrite[0]"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn custom_consistent_hour_representations_preserve_midnight_and_noon_distances() {
    for (format, text, expected) in [
        (
            "%H:%M:%S %p",
            "00:00:00 AM|12:00:00 PM",
            "12:00:00 PM|00:00:00 AM",
        ),
        (
            "%p %H:%M:%S",
            "AM 00:00:00|PM 12:00:00",
            "PM 12:00:00|AM 00:00:00",
        ),
        (
            "%I %p (%H:%M:%S)",
            "12 AM (00:00:00)|12 PM (12:00:00)",
            "12 PM (12:00:00)|12 AM (00:00:00)",
        ),
        (
            "%H:%M:%S (%I %p)",
            "00:00:00 (12 AM)|12:00:00 (12 PM)",
            "12:00:00 (12 PM)|00:00:00 (12 AM)",
        ),
        (
            "%I (%H:%M:%S)",
            "12 (00:00:00)|12 (12:00:00)",
            "12 (12:00:00)|12 (00:00:00)",
        ),
        (
            "%l %P (%k:%M:%S)",
            "12 am ( 0:00:00)|12 pm (12:00:00)",
            "12 pm (12:00:00)|12 am ( 0:00:00)",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
}

#[test]
fn custom_epoch_reconciliation_checks_ordinal_week_quarter_and_offset_fields() {
    for (format, text, expected) in [
        (
            "%s %Y-%j %T",
            "0 2026-071 12:00:00|86400 2026-071 13:00:00",
            "3386750400 2077-117 12:00:00|3386754000 2077-117 13:00:00",
        ),
        (
            "%s %G-W%V-%u %T",
            "0 1970-W01-4 00:00:00|90000 1970-W01-5 01:00:00",
            "3386750400 2077-W17-2 12:00:00|3386840400 2077-W17-3 13:00:00",
        ),
        (
            "%s %Y %U %w %T",
            "0 1970 00 4 12:00:00|86400 1970 00 5 13:00:00",
            "3386750400 2077 17 2 12:00:00|3386754000 2077 17 2 13:00:00",
        ),
        (
            "%s %Y %W %w %T",
            "0 1970 00 4 12:00:00|86400 1970 00 5 13:00:00",
            "3386750400 2077 17 2 12:00:00|3386754000 2077 17 2 13:00:00",
        ),
        (
            "%s %Y-%m-%d %q %T",
            "0 1970-01-01 2 12:00:00|86400 1970-01-02 2 13:00:00",
            "3386750400 2077-04-27 2 12:00:00|3386754000 2077-04-27 2 13:00:00",
        ),
        (
            "%s %C %T",
            "0 19 00:00:00|90000 19 01:00:00",
            "3386750400 20 12:00:00|3386840400 20 13:00:00",
        ),
        (
            "%s%.f %T",
            "0.1 12:00:00|86400.2 13:00:00",
            "3386750400 12:00:00|3386754000.100 13:00:00",
        ),
        (
            "%s %Y%m%d %T %z",
            "0 19700101 00:00:00 +0545|90000 19700102 01:00:00 -0330",
            "3386750400 20770427 12:00:00 +0000|3386840400 20770428 13:00:00 +0000",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
}

#[test]
fn custom_hour_constraints_reconcile_all_directive_permutations() {
    for order in [
        ["%I", "%p", "%H"],
        ["%I", "%H", "%p"],
        ["%p", "%I", "%H"],
        ["%p", "%H", "%I"],
        ["%H", "%I", "%p"],
        ["%H", "%p", "%I"],
    ] {
        let prefix = |hour12, hour24| {
            order
                .map(|part| match part {
                    "%I" => hour12,
                    "%H" => hour24,
                    _ => "PM",
                })
                .join(" ")
        };
        let format = format!("{}:%M:%S", order.join(" "));
        let text = format!("{}:34:56|{}:34:56", prefix("01", "13"), prefix("02", "14"));
        let expected = format!("{}:00:00|{}:00:00", prefix("12", "12"), prefix("01", "13"));
        assert_custom_output(&text, &format, &expected);
        let (output, _) = rewrite(
            &format!("{}:34:56", prefix("01", "12")),
            json!({"pattern":"[^|\\n]+", "custom_format": format}),
        );
        assert_eq!(output.status.code(), Some(1));
    }
}

#[test]
fn custom_iso_two_digit_year_validates_complete_dates_before_applying_pivot() {
    assert_custom_output(
        "1969-06-01 12:00:00 69|1969-06-02 12:00:00 69",
        "%Y-%m-%d %T %g",
        "2077-04-27 12:00:00 77|2077-04-28 12:00:00 77",
    );
    assert_custom_output(
        "69 1969-06-01 12:00:00|69 1969-06-02 12:00:00",
        "%g %Y-%m-%d %T",
        "77 2077-04-27 12:00:00|77 2077-04-28 12:00:00",
    );
    assert_custom_output(
        "1969-152 12:00:00 69|1969-153 12:00:00 69",
        "%Y-%j %T %g",
        "2077-117 12:00:00 77|2077-118 12:00:00 77",
    );
    assert_custom_output(
        "1969-06-01 12:00:00 68|1969-06-02 12:00:00 68",
        "%Y-%m-%d %T %g",
        "2077-04-27 12:00:00 77|2077-04-27 12:00:00 77",
    );
    assert_custom_output(
        "6292987200 69|6293073600 69",
        "%s %g",
        "3386750400 77|3386836800 77",
    );
}

#[test]
fn custom_gregorian_two_digit_year_validates_complete_iso_week_dates() {
    for (format, text, expected) in [
        (
            "%y %G-W%V-%u %T",
            "69 1969-W22-7 12:00:00|69 1969-W23-1 12:00:00",
            "77 2077-W17-2 12:00:00|77 2077-W17-3 12:00:00",
        ),
        (
            "%G-W%V-%u %T %y",
            "1969-W22-7 12:00:00 69|1969-W23-1 12:00:00 69",
            "2077-W17-2 12:00:00 77|2077-W17-3 12:00:00 77",
        ),
        (
            "%y %G-W%V-%u %T",
            "68 1969-W22-7 12:00:00|68 1969-W23-1 12:00:00",
            "77 2077-W17-2 12:00:00|77 2077-W17-2 12:00:00",
        ),
        (
            "%y %G-W%V-%u %T",
            "15 2015-W53-4 12:00:00|16 2015-W53-5 12:00:00",
            "77 2077-W17-2 12:00:00|77 2077-W17-3 12:00:00",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
}

#[test]
fn custom_non_iso_week_dates_validate_iso_year_without_pivoting() {
    for (format, text, expected) in [
        (
            "%Y %U %w %g %V %T",
            "1969 22 0 69 22 12:00:00|1969 22 1 69 23 12:00:00",
            "2077 17 2 77 17 12:00:00|2077 17 3 77 17 12:00:00",
        ),
        (
            "%Y %W %w %g %V %T",
            "1969 21 0 69 22 12:00:00|1969 22 1 69 23 12:00:00",
            "2077 17 2 77 17 12:00:00|2077 17 3 77 17 12:00:00",
        ),
        (
            "%Y %U %w %g %V %T",
            "1969 22 0 68 22 12:00:00|1969 22 1 68 23 12:00:00",
            "2077 17 2 77 17 12:00:00|2077 17 2 77 17 12:00:00",
        ),
    ] {
        assert_custom_output(text, format, expected);
    }
}
