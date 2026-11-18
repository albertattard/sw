//! Compatibility adapter for the custom formats accepted before TASK-208.
#[path = "datetime_fields.rs"]
mod fields;
use super::{FixedDatetime, RenderError, operational};
use fields::{HourComponents, ParsedDatetime, YearComponents};
use jiff::fmt::strtime::{self, BrokenDownTime};
use std::collections::HashMap;

#[derive(Debug)]
enum FormatToken {
    Literal(char),
    Directive { format: String, kind: char },
}

/// Expand composites before adapting directives. Never hand an unknown directive
/// to Jiff: it has additional directives which were not part of our contract.
fn format_tokens(format: &str) -> Result<Vec<FormatToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            tokens.push(FormatToken::Literal(c));
            continue;
        }
        let mut token = String::from("%");
        let directive = loop {
            let c = chars.next().ok_or("incomplete format directive")?;
            token.push(c);
            if !matches!(c, '-' | '_' | '0'..='9' | '.' | ':' | '#') {
                break c;
            }
        };
        let composite = match token.as_str() {
            "%+" => Some("%Y-%m-%dT%H:%M:%S%.f%:z"),
            "%c" => Some("%a %b %e %H:%M:%S %Y"),
            "%x" | "%D" => Some("%m/%d/%y"),
            "%X" | "%T" => Some("%H:%M:%S"),
            "%r" => Some("%I:%M:%S %p"),
            "%R" => Some("%H:%M"),
            "%F" => Some("%Y-%m-%d"),
            "%v" => Some("%e-%b-%Y"),
            _ => None,
        };
        if let Some(expansion) = composite {
            let mut expanded = format_tokens(expansion)?;
            if token == "%+" {
                for part in &mut expanded {
                    match part {
                        FormatToken::Literal('T') => {
                            *part = FormatToken::Directive {
                                format: "%~T".into(),
                                kind: 'T',
                            }
                        }
                        FormatToken::Directive { format, .. } if format == "%:z" => {
                            *format = "%~z".into();
                        }
                        _ => {}
                    }
                }
            }
            tokens.extend(expanded);
        } else {
            if !"AaBbhCdegGHIjklmMpPqSsuUVwWyYzZfn t%".contains(directive)
                || !valid_token(&token, directive)
            {
                return Err(format!("unsupported format directive `{token}`"));
            }
            tokens.push(if directive == '%' {
                FormatToken::Literal('%')
            } else {
                FormatToken::Directive {
                    format: token,
                    kind: directive,
                }
            });
        }
    }
    Ok(tokens)
}

fn valid_token(token: &str, directive: char) -> bool {
    match directive {
        'f' => matches!(
            token,
            "%f" | "%_f" | "%-f" | "%0f" | "%3f" | "%6f" | "%9f" | "%.f" | "%.3f" | "%.6f" | "%.9f"
        ),
        'z' => matches!(token, "%z" | "%:z" | "%::z" | "%:::z" | "%#z"),
        _ => {
            token.len() == 2
                || (token.len() == 3
                    && matches!(token.as_bytes()[1], b'-' | b'_' | b'0')
                    && "CdegGHIjklmMqSsUuVWwYy".contains(directive))
        }
    }
}

/// Normalize fractions using consumed input positions, not string replacement:
/// raw %f is a nanosecond integer, while Jiff reads a decimal fraction. Preserve
/// fixed-width input validation and let the final parse check field consistency.
pub(super) fn parse_custom(value: &str, format: &str) -> Result<ParsedDatetime, String> {
    let tokens = format_tokens(format)?;
    let mut rest = value;
    let mut normalized_value = String::new();
    let mut normalized_format = String::new();
    let mut seen = HashMap::new();
    let mut year = YearComponents::default();
    let mut iso_year = YearComponents::default();
    let mut hours = HourComponents::default();
    let mut quarter = None;
    for token in tokens {
        // Separate adjacent numeric fields after normalization so a variable
        // width timestamp or fraction cannot consume the following field.
        normalized_format.push('\0');
        normalized_value.push('\0');
        let (token, kind) = match token {
            FormatToken::Literal(literal) => {
                if literal.is_whitespace() {
                    rest = rest.trim_start_matches(char::is_whitespace);
                } else {
                    rest = rest
                        .strip_prefix(literal)
                        .ok_or_else(|| format!("expected literal `{literal}`"))?;
                    if literal == '%' {
                        normalized_format.push('%');
                    }
                    normalized_format.push(literal);
                    normalized_value.push(literal);
                }
                continue;
            }
            FormatToken::Directive { format, kind } => (format, kind),
        };
        if matches!(kind, 'n' | 't') {
            rest = rest.trim_start_matches(char::is_whitespace);
            continue;
        }
        if kind == 'f' {
            let dotted = token.starts_with("%.");
            if !dotted {
                rest = rest.trim_start();
            }
            if dotted && !rest.starts_with('.') {
                continue;
            }
            if dotted {
                rest = &rest[1..];
            }
            let width = token
                .bytes()
                .find(|c| matches!(c, b'3' | b'6' | b'9'))
                .map(|c| usize::from(c - b'0'));
            let count = rest
                .bytes()
                .take_while(u8::is_ascii_digit)
                .count()
                .min(width.unwrap_or(9));
            if count == 0 || width.is_some_and(|w| count != w) {
                return Err(format!("invalid fractional width for `{token}`"));
            }
            let digits = &rest[..count];
            let nanos = if matches!(token.as_str(), "%f" | "%_f" | "%-f" | "%0f") {
                format!("{:0>9}", digits)
            } else {
                format!("{digits:0<9}")
            };
            remember(&mut seen, "fraction", nanos.clone())?;
            normalized_format.push_str("%9f");
            normalized_value.push_str(&nanos);
            rest = &rest[count..];
            if token == "%.f" {
                rest = rest.trim_start_matches(|c: char| c.is_ascii_digit());
            }
        } else if token == "%~T" {
            if !rest.starts_with(['T', 't', ' ']) {
                return Err("expected RFC 3339 separator".into());
            }
            rest = &rest[1..];
            normalized_format.push('T');
            normalized_value.push('T');
        } else if matches!(token.as_str(), "%q" | "%_q" | "%-q" | "%0q") {
            rest = rest.trim_start();
            let digit = rest.as_bytes().first().copied().ok_or("missing quarter")?;
            if !(b'1'..=b'4').contains(&digit) {
                return Err("invalid quarter".into());
            }
            remember(&mut seen, "quarter", char::from(digit).to_string())?;
            quarter = Some((digit - b'0') as i8);
            normalized_format.push(char::from(digit));
            normalized_value.push(char::from(digit));
            rest = &rest[1..];
        } else if token == "%Z" {
            // Naive Chrono parsing consumes timezone names without resolving
            // them. Fixed-offset formatting below emits the base offset.
            let count = rest.find(char::is_whitespace).unwrap_or(rest.len());
            normalized_format.push_str(&rest[..count].replace('%', "%%"));
            normalized_value.push_str(&rest[..count]);
            rest = &rest[count..];
        } else if kind == 'z' {
            let (remaining, offset) = parse_offset(rest, &token)?;
            remember(&mut seen, "offset", offset.to_string())?;
            normalized_format.push_str("%:z");
            normalized_value.push_str(&format!(
                "{}{:02}:{:02}",
                if offset < 0 { '-' } else { '+' },
                offset.abs() / 3600,
                offset.abs() / 60 % 60
            ));
            rest = remaining;
        } else {
            if kind == 's' && !rest.trim_start().starts_with(|c: char| c.is_ascii_digit()) {
                return Err("invalid Unix timestamp".into());
            }
            let parse_token = token.as_str();
            let parse_input = rest;
            // Chrono accepts full or abbreviated names for %B and %A,
            // retaining their full-name output. Prefer the longest name first.
            let (piece, consumed, parse_token) = if matches!(kind, 'B' | 'A') {
                let (full, abbreviated) = if kind == 'B' {
                    ("%B", "%b")
                } else {
                    ("%A", "%a")
                };
                match BrokenDownTime::parse_prefix(full, parse_input) {
                    Ok((piece, consumed)) => (piece, consumed, full),
                    Err(_) => {
                        let (piece, consumed) =
                            BrokenDownTime::parse_prefix(abbreviated, parse_input)
                                .map_err(|e| e.to_string())?;
                        (piece, consumed, abbreviated)
                    }
                }
            } else {
                let (piece, consumed) = BrokenDownTime::parse_prefix(parse_token, parse_input)
                    .map_err(|e| e.to_string())?;
                (piece, consumed, parse_token)
            };
            if matches!(kind, 'C' | 'Y' | 'y' | 'G' | 'g') {
                let consumed_value = &rest[..consumed];
                if matches!(kind, 'C' | 'y' | 'g')
                    && !consumed_value
                        .trim_start()
                        .starts_with(|c: char| c.is_ascii_digit())
                {
                    return Err("invalid unsigned year component".into());
                }
                match kind {
                    'C' => year.set_century(piece.year().expect("%C sets a year") / 100)?,
                    'Y' => year.set_full(piece.year().expect("%Y sets a year"))?,
                    'y' => year.set_two_digit(piece.year().expect("%y sets a year") % 100)?,
                    'G' => {
                        iso_year.set_full(piece.iso_week_year().expect("%G sets an ISO year"))?
                    }
                    'g' => iso_year
                        .set_two_digit(piece.iso_week_year().expect("%g sets an ISO year") % 100)?,
                    _ => unreachable!(),
                }
                // Do not let Jiff overwrite earlier components or apply its
                // own two-digit-year pivot in the final normalized parse.
                rest = &rest[consumed..];
                continue;
            }
            if piece
                .offset()
                .is_some_and(|offset| offset.seconds().abs() >= 86_400)
            {
                return Err("UTC offset must be strictly within 24 hours".into());
            }
            for (name, field) in [
                ("iso-week", piece.iso_week().map(i64::from)),
                ("sunday-week", piece.sunday_based_week().map(i64::from)),
                ("monday-week", piece.monday_based_week().map(i64::from)),
                ("month", piece.month().map(i64::from)),
                ("day", piece.day().map(i64::from)),
                ("ordinal", piece.day_of_year().map(i64::from)),
                ("minute", piece.minute().map(i64::from)),
                ("second", piece.second().map(i64::from)),
                ("offset", piece.offset().map(|v| i64::from(v.seconds()))),
                ("epoch", piece.timestamp().map(|v| v.as_second())),
            ] {
                if let Some(field) = field {
                    remember(&mut seen, name, field.to_string())?;
                }
            }
            if let Some(weekday) = piece.weekday() {
                remember(&mut seen, "weekday", format!("{weekday:?}"))?;
            }
            if matches!(kind, 'H' | 'k' | 'I' | 'l' | 'p' | 'P') {
                hours.record(kind, &piece)?;
                // Jiff adjusts hours as each directive arrives. Retain the
                // independent half-day/remainder constraints until conversion.
                rest = &rest[consumed..];
                continue;
            }
            let consumed_value = &rest[..consumed];
            if kind == 'S' && consumed_value.trim().parse::<u8>() == Ok(60) {
                return Err("leap seconds are not supported".into());
            }
            normalized_format.push_str(parse_token);
            normalized_value.push_str(consumed_value);
            rest = &rest[consumed..];
        }
    }
    if !rest.is_empty() {
        return Err("trailing input".into());
    }
    let parsed = strtime::parse(normalized_format, normalized_value).map_err(|e| e.to_string())?;
    ParsedDatetime::new(parsed, year, iso_year, hours, quarter)
}

fn parse_offset<'a>(input: &'a str, token: &str) -> Result<(&'a str, i32), String> {
    let mut rest = input.trim_start();
    if token == "%~z" && rest.get(..3).is_some_and(|v| v.eq_ignore_ascii_case("UTC")) {
        return Ok((&rest[3..], 0));
    }
    if matches!(token, "%~z" | "%#z") && rest.starts_with(['Z', 'z']) {
        return Ok((&rest[1..], 0));
    }
    let sign = match rest.chars().next() {
        Some('+') => 1,
        Some('-' | '−') => -1,
        _ => return Err("invalid numeric UTC offset".into()),
    };
    rest = &rest[rest.chars().next().unwrap().len_utf8()..];
    let hours = rest
        .get(..2)
        .filter(|v| v.bytes().all(|b| b.is_ascii_digit()))
        .ok_or("invalid numeric UTC offset")?
        .parse::<i32>()
        .map_err(|e| e.to_string())?;
    rest = rest[2..].trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    let minutes = if let Some(digits) = rest
        .get(..2)
        .filter(|v| v.bytes().all(|b| b.is_ascii_digit()))
    {
        rest = &rest[2..];
        digits.parse::<i32>().map_err(|e| e.to_string())?
    } else if token == "%#z" && rest.is_empty() {
        0
    } else {
        return Err("invalid numeric UTC offset".into());
    };
    if hours >= 24 || minutes >= 60 {
        return Err("invalid numeric UTC offset".into());
    }
    Ok((rest, sign * (hours * 3600 + minutes * 60)))
}

fn remember(
    seen: &mut HashMap<&'static str, String>,
    field: &'static str,
    value: String,
) -> Result<(), String> {
    if let Some(previous) = seen.insert(field, value.clone())
        && previous != value
    {
        return Err(format!("conflicting {field} fields"));
    }
    Ok(())
}

pub(super) fn format_custom(datetime: FixedDatetime, format: &str) -> Result<String, RenderError> {
    let tokens = format_tokens(format).map_err(operational)?;
    let mut tm = BrokenDownTime::from(datetime.local);
    tm.set_offset(Some(datetime.offset));
    let mut output = String::new();
    for token in tokens {
        let token = match token {
            FormatToken::Literal(literal) => {
                output.push(literal);
                continue;
            }
            FormatToken::Directive { format, .. } => format,
        };
        match token.as_str() {
            "%f" | "%0f" => output.push_str(&format!("{:09}", datetime.local.subsec_nanosecond())),
            "%_f" => output.push_str(&format!("{:9}", datetime.local.subsec_nanosecond())),
            "%-f" => output.push_str(&datetime.local.subsec_nanosecond().to_string()),
            "%~T" => output.push('T'),
            "%~z" => output.push_str(&tm.to_string("%:z").map_err(operational)?),
            "%.f" => {
                let nanos = datetime.local.subsec_nanosecond();
                let width = if nanos == 0 {
                    0
                } else if nanos % 1_000_000 == 0 {
                    3
                } else if nanos % 1_000 == 0 {
                    6
                } else {
                    9
                };
                if width != 0 {
                    output.push('.');
                    output.push_str(&format!("{nanos:09}")[..width]);
                }
            }
            "%C" => output.push_str(&tm.to_string("%02C").map_err(operational)?),
            "%Z" => output.push_str(&tm.to_string("%:z").map_err(operational)?),
            "%:::z" => {
                let seconds = datetime.offset.seconds();
                output.push_str(&format!(
                    "{}{:02}",
                    if seconds < 0 { '-' } else { '+' },
                    seconds.abs() / 3600
                ));
            }
            "%#z" => return Err(operational("unsupported output format directive `%#z`")),
            _ => output.push_str(&tm.to_string(token).map_err(operational)?),
        }
    }
    Ok(output)
}
