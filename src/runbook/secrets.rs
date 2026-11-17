use super::{RenderError, ValidationIssue};
use serde_json::Value;
use std::collections::HashMap;

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

#[derive(Clone, Default)]
pub(crate) struct Secrets(Vec<(String, String)>);

impl Secrets {
    pub(crate) fn generate(runbook: &Value) -> Result<Self, RenderError> {
        Self::generate_with(runbook, |bytes| {
            getrandom::fill(bytes).map_err(|error| error.to_string())
        })
    }

    fn generate_with(
        runbook: &Value,
        mut fill: impl FnMut(&mut [u8]) -> Result<(), String>,
    ) -> Result<Self, RenderError> {
        let mut values = Vec::new();
        if let Some(declarations) = runbook.get("secrets").and_then(Value::as_object) {
            for (name, declaration) in declarations {
                let length = declaration["generate"]["length"].as_u64().unwrap_or(32) as usize;
                let mut value = String::with_capacity(length);
                while value.len() < length {
                    let mut bytes = [0u8; 128];
                    fill(&mut bytes).map_err(|error| {
                        RenderError::Operational(format!("Failed to generate secrets: {error}"))
                    })?;
                    // Reject the incomplete final bucket to avoid modulo bias.
                    for byte in bytes.into_iter().filter(|byte| *byte < 248) {
                        value.push(ALPHABET[byte as usize % ALPHABET.len()] as char);
                        if value.len() == length {
                            break;
                        }
                    }
                }
                values.push((name.clone(), value));
            }
        }
        Ok(Self(values))
    }

    pub(crate) fn reveal(&self) {
        if self.0.is_empty() {
            return;
        }
        eprintln!("Generated secrets:");
        for (name, value) in &self.0 {
            eprintln!("  {name}: {value}");
        }
    }

    pub(crate) fn environment(&self, entry: &Value) -> HashMap<String, String> {
        entry
            .get("environment")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
            .filter_map(|(name, binding)| {
                let reference = binding.get("secret")?.as_str()?;
                self.0
                    .iter()
                    .find(|(key, _)| key == reference)
                    .map(|(_, value)| (name.clone(), value.clone()))
            })
            .collect()
    }

    pub(crate) fn redact(&self, text: &str) -> String {
        // Match against the original text, preferring longer values at the same position.
        let mut result = String::new();
        let mut rest = text;
        while !rest.is_empty() {
            let next = self
                .0
                .iter()
                .filter_map(|(_, value)| rest.find(value).map(|start| (start, value.len())))
                .min_by_key(|(start, length)| (*start, std::cmp::Reverse(*length)));
            let Some((start, length)) = next else {
                result.push_str(rest);
                break;
            };
            result.push_str(&rest[..start]);
            result.push_str("******");
            rest = &rest[start + length..];
        }
        result
    }

    pub(crate) fn redact_error(&self, error: RenderError) -> RenderError {
        match error {
            RenderError::Operational(message) => RenderError::Operational(self.redact(&message)),
            RenderError::CommandFailed(message) => {
                RenderError::CommandFailed(self.redact(&message))
            }
            RenderError::CleanupFailed { message, markdown } => RenderError::CleanupFailed {
                message: self.redact(&message),
                markdown: self.redact(&markdown),
            },
            RenderError::Timeout {
                message,
                partial_markdown,
            } => RenderError::Timeout {
                message: self.redact(&message),
                partial_markdown: self.redact(&partial_markdown),
            },
            RenderError::FailedAt {
                entry_number,
                error,
            } => RenderError::FailedAt {
                entry_number,
                error: Box::new(self.redact_error(*error)),
            },
        }
    }
}

fn valid_name(name: &str) -> bool {
    let mut chars = name.bytes();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == b'_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == b'_')
}

pub(super) fn validate(runbook: &Value, errors: &mut Vec<ValidationIssue>) {
    let mut error = |path: String, message: &str| {
        errors.push(ValidationIssue {
            path,
            message: message.into(),
        })
    };
    let declarations = runbook.get("secrets").and_then(Value::as_object);
    if runbook.get("secrets").is_some() && declarations.is_none() {
        error("$.secrets".into(), "must be an object");
    }
    for (name, declaration) in declarations.into_iter().flatten() {
        let path = format!("$.secrets.{name}");
        if !valid_name(name) {
            error(path.clone(), "invalid secret name");
        }
        let Some(fields) = declaration.as_object() else {
            error(path, "must be an object");
            continue;
        };
        for key in fields.keys().filter(|key| *key != "generate") {
            error(format!("{path}.{key}"), "unknown secret property");
        }
        let Some(generate) = fields.get("generate").and_then(Value::as_object) else {
            error(
                format!("{path}.generate"),
                "must be an object and is required",
            );
            continue;
        };
        for key in generate.keys().filter(|key| *key != "length") {
            error(
                format!("{path}.generate.{key}"),
                "unknown generation property",
            );
        }
        if let Some(length) = generate.get("length")
            && !length.as_u64().is_some_and(|n| (16..=256).contains(&n))
        {
            error(
                format!("{path}.generate.length"),
                "must be an integer from 16 through 256",
            );
        }
    }
    for (index, entry) in runbook
        .get("entries")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let Some(environment) = entry.get("environment") else {
            continue;
        };
        let path = format!("$.entries[{index}].environment");
        if entry["type"] != "Command" {
            error(path.clone(), "only Command entries support environment");
        }
        let Some(bindings) = environment.as_object() else {
            error(path, "must be an object");
            continue;
        };
        for (name, binding) in bindings {
            let path = format!("{path}.{name}");
            if !valid_name(name) {
                error(path.clone(), "invalid environment variable name");
            }
            let Some(fields) = binding.as_object() else {
                error(path, "must be an object");
                continue;
            };
            for key in fields.keys().filter(|key| *key != "secret") {
                error(format!("{path}.{key}"), "unknown binding property");
            }
            if !fields
                .get("secret")
                .and_then(Value::as_str)
                .is_some_and(|name| declarations.is_some_and(|d| d.contains_key(name)))
            {
                error(format!("{path}.secret"), "must reference a declared secret");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn generation_rejects_biased_bytes_and_propagates_failure() {
        let document =
            json!({"secrets": {"a": {"generate": {"length": 62}}, "b": {"generate": {}}}});
        let secrets = Secrets::generate_with(&document, |bytes| {
            for (i, byte) in bytes.iter_mut().enumerate() {
                *byte = (i + 248) as u8;
            }
            Ok(())
        })
        .unwrap_or_else(|_| panic!("generation failed"));
        assert_eq!(secrets.0[0].1.as_bytes(), ALPHABET);
        let mut calls = 0;
        assert!(
            Secrets::generate_with(&document, |bytes| {
                calls += 1;
                if calls > 1 {
                    return Err("unavailable".into());
                }
                bytes.fill(0);
                Ok(())
            })
            .is_err()
        );
    }
}
