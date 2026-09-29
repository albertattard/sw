use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn parse_capture_value(rule: &Value, captured: &str) -> Result<Option<f64>, String> {
    let Some(parse_as) = rule.get("parse_as") else {
        return Ok(None);
    };
    let parse_as = parse_as
        .as_object()
        .ok_or_else(|| "parse_as must be an object".to_string())?;
    if parse_as.get("type").and_then(Value::as_str) != Some("number") {
        return Err("parse_as.type must be `number`".to_string());
    }

    let system_locale_value;
    let (decimal_separator, grouping_separator) = if let Some(locale) = parse_as.get("locale") {
        let locale = locale
            .as_str()
            .ok_or_else(|| "parse_as.locale must be a string".to_string())?;
        separators_for_locale(if locale == "system" {
            system_locale_value = system_locale();
            &system_locale_value
        } else {
            locale
        })?
    } else {
        let decimal = parse_as
            .get("decimal_separator")
            .and_then(Value::as_str)
            .unwrap_or(".");
        let grouping = match parse_as.get("grouping_separator").and_then(Value::as_str) {
            Some("none") | None => "",
            Some(value) => value,
        };
        (decimal, grouping)
    };

    parse_number(captured, decimal_separator, grouping_separator).map(Some)
}

fn system_locale() -> String {
    ["LC_ALL", "LC_NUMERIC", "LANG"]
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
        .unwrap_or_else(|| "C".to_string())
}

fn separators_for_locale(locale: &str) -> Result<(&'static str, &'static str), String> {
    let language = locale
        .split(['-', '_', '.'])
        .next()
        .unwrap_or(locale)
        .to_ascii_lowercase();
    match language.as_str() {
        "c" | "posix" => Ok((".", "")),
        "en" => Ok((".", ",")),
        "de" | "es" | "it" | "nl" | "pt" => Ok((",", ".")),
        "fr" | "ru" => Ok((",", "\u{202f}")),
        _ => Err(format!("unsupported numeric parse locale `{locale}`")),
    }
}

fn parse_number(
    input: &str,
    decimal_separator: &str,
    grouping_separator: &str,
) -> Result<f64, String> {
    if decimal_separator.chars().count() != 1
        || (!grouping_separator.is_empty() && grouping_separator.chars().count() != 1)
        || decimal_separator == grouping_separator
    {
        return Err("numeric separators must be distinct single characters".to_string());
    }

    let input = input.trim();
    if input.is_empty() {
        return Err("numeric value is empty".to_string());
    }
    let (sign, magnitude) = match input.as_bytes()[0] {
        b'+' => ("+", &input[1..]),
        b'-' => ("-", &input[1..]),
        _ => ("", input),
    };
    if magnitude.is_empty() {
        return Err(format!("`{input}` is not a number"));
    }

    let decimal_parts: Vec<&str> = magnitude.split(decimal_separator).collect();
    if decimal_parts.len() > 2 || decimal_parts.iter().any(|part| part.is_empty()) {
        return Err(format!("`{input}` is not a number"));
    }
    let integer = decimal_parts[0];
    let fraction = decimal_parts.get(1).copied();
    if !integer.chars().all(|character| character.is_ascii_digit()) && grouping_separator.is_empty()
    {
        return Err(format!("`{input}` is not a number"));
    }
    if let Some(fraction) = fraction
        && !fraction.chars().all(|character| character.is_ascii_digit())
    {
        return Err(format!("`{input}` is not a number"));
    }

    let normalized_integer = if grouping_separator.is_empty() {
        integer.to_string()
    } else {
        normalize_grouped_integer(integer, grouping_separator)
            .ok_or_else(|| format!("`{input}` is not a number"))?
    };
    let mut normalized = format!("{sign}{normalized_integer}");
    if let Some(fraction) = fraction {
        normalized.push('.');
        normalized.push_str(fraction);
    }
    normalized
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("`{input}` is not a finite number"))
}

fn normalize_grouped_integer(input: &str, separator: &str) -> Option<String> {
    let groups: Vec<&str> = input.split(separator).collect();
    if groups.len() == 1 {
        return groups[0]
            .chars()
            .all(|character| character.is_ascii_digit())
            .then(|| groups[0].to_string());
    }
    if groups[0].is_empty()
        || groups[0].len() > 3
        || !groups[0]
            .chars()
            .all(|character| character.is_ascii_digit())
        || groups[1..].iter().any(|group| {
            group.len() != 3 || !group.chars().all(|character| character.is_ascii_digit())
        })
    {
        return None;
    }
    Some(groups.concat())
}

pub(crate) fn evaluate_expression(
    expression: &str,
    numeric_values: &HashMap<String, f64>,
) -> Result<String, String> {
    let mut parser = ExpressionParser::new(expression, numeric_values);
    let value = parser.expression()?;
    parser.skip_whitespace();
    if !value.is_finite() {
        Err("arithmetic result is not finite".to_string())
    } else if parser.remaining().is_empty() {
        Ok(value.to_string())
    } else {
        Err(format!("unexpected token near `{}`", parser.remaining()))
    }
}

struct ExpressionParser<'a> {
    input: &'a str,
    position: usize,
    numeric_values: &'a HashMap<String, f64>,
}

impl<'a> ExpressionParser<'a> {
    fn new(input: &'a str, numeric_values: &'a HashMap<String, f64>) -> Self {
        Self {
            input,
            position: 0,
            numeric_values,
        }
    }

    fn expression(&mut self) -> Result<f64, String> {
        let mut value = self.term()?;
        loop {
            self.skip_whitespace();
            match self.peek_char() {
                Some('+') => {
                    self.consume_char();
                    value += self.term()?;
                }
                Some('-') => {
                    self.consume_char();
                    value -= self.term()?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut value = self.factor()?;
        loop {
            self.skip_whitespace();
            match self.peek_char() {
                Some('*') => {
                    self.consume_char();
                    value *= self.factor()?;
                }
                Some('/') => {
                    self.consume_char();
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("division by zero".to_string());
                    }
                    value /= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    fn factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        match self.consume_char() {
            Some('+') => self.factor(),
            Some('-') => Ok(-self.factor()?),
            Some('(') => {
                let value = self.expression()?;
                self.skip_whitespace();
                if self.consume_char() == Some(')') {
                    Ok(value)
                } else {
                    Err("expected `)`".to_string())
                }
            }
            Some(character) if character.is_ascii_digit() || character == '.' => {
                self.position -= character.len_utf8();
                self.number()
            }
            Some(character) if character.is_ascii_alphabetic() || character == '_' => {
                self.position -= character.len_utf8();
                let name = self.identifier();
                self.numeric_values
                    .get(&name)
                    .copied()
                    .ok_or_else(|| format!("`{name}` has no parsed numeric value"))
            }
            Some(character) => Err(format!("unexpected character `{character}`")),
            None => Err("expected a number, variable, or `(`".to_string()),
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        let start = self.position;
        while self
            .peek_char()
            .is_some_and(|character| character.is_ascii_digit() || character == '.')
        {
            self.consume_char();
        }
        self.input[start..self.position]
            .parse::<f64>()
            .map_err(|_| "invalid numeric literal".to_string())
    }

    fn identifier(&mut self) -> String {
        let start = self.position;
        while self
            .peek_char()
            .is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
        {
            self.consume_char();
        }
        self.input[start..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().is_some_and(char::is_whitespace) {
            self.consume_char();
        }
    }
    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    fn consume_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.position += character.len_utf8();
        Some(character)
    }
    fn remaining(&self) -> &str {
        &self.input[self.position..]
    }
}
