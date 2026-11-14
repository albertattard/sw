use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static ID: AtomicU64 = AtomicU64::new(0);
struct Workspace(PathBuf);
impl Workspace {
    fn new(document: Value) -> Self {
        let path = std::env::temp_dir().join(format!(
            "sw-secrets-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("sw-runbook.json"), document.to_string()).unwrap();
        Self(path)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_sw"))
            .args(args)
            .current_dir(&self.0)
            .env("PASSWORD", "inherited")
            .env_remove("ALIAS")
            .output()
            .unwrap()
    }
    fn read(&self, name: &str) -> String {
        fs::read_to_string(self.0.join(name)).unwrap()
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn binding() -> Value {
    json!({"PASSWORD":{"secret":"password"}})
}
fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
fn reveals(output: &Output) -> Vec<(String, String)> {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .skip_while(|line| *line != "Generated secrets:")
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .map(|line| {
            let (key, value) = line.trim_start().split_once(": ").unwrap();
            (key.into(), value.into())
        })
        .collect()
}

#[test]
fn generated_values_are_shared_bound_scoped_and_renewed() {
    let workspace = Workspace::new(
        json!({"secrets":{"password":{"generate":{}},"other":{"generate":{"length":256}}},"entries":[
            {"type":"Command","environment":binding(),"commands":"printf '%s' \"$PASSWORD\" > first; printf '%s' \"$PASSWORD\"", "cleanup":"printf '%s' \"$PASSWORD\" > cleanup", "output":{}},
            {"type":"Command","environment":{"ALIAS":{"secret":"password"}},"commands":"printf '%s' \"$ALIAS\" > second; test \"$PASSWORD\" = inherited"},
            {"type":"Command","commands":"test -z \"$ALIAS\"; test \"$PASSWORD\" = inherited"}
        ]}),
    );
    let output = workspace.run(&["run", "--reveal-secrets"]);
    assert_success(&output);
    let list = reveals(&output);
    assert_eq!(
        list.iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        ["password", "other"]
    );
    for (index, length) in [32, 256].iter().enumerate() {
        assert_eq!(list[index].1.len(), *length);
        assert!(list[index].1.bytes().all(|c| c.is_ascii_alphanumeric()));
    }
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "Generated secrets:\n  password: {}\n  other: {}\n",
            list[0].1, list[1].1
        )
    );
    assert_ne!(list[0].1, list[1].1);
    for path in ["first", "second", "cleanup"] {
        assert_eq!(workspace.read(path), list[0].1);
    }
    assert!(!workspace.read("README.md").contains(&list[0].1));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("[debug]"));
    let previous = list[0].1.clone();
    assert_success(&workspace.run(&["run", "--start-at", "1"]));
    assert_ne!(previous, workspace.read("first"));
}

#[test]
fn captures_assertions_debug_and_split_streams_keep_real_values_but_mask_output() {
    let workspace = Workspace::new(
        json!({"secrets":{"password":{"generate":{"length":16}}},"entries":[
            {"type":"Command","environment":binding(),"commands":"printf '%s' \"$PASSWORD\" > actual; a=$(printf '%s' \"$PASSWORD\" | cut -c1-8); b=$(printf '%s' \"$PASSWORD\" | cut -c9-); printf '%s' \"$a\"; sleep 0.15; printf '%s\\n' \"$b\"; printf '%s\\n' \"$PASSWORD\" >&2", "output":{"stream":"combined"},
             "capture":[{"name":"captured","source":"stdout","stage":"raw","pattern":"([A-Za-z0-9]{16})"}],
             "assert":{"exit_code":0},
             "cleanup":"printf '%s\\n' \"$PASSWORD\"; printf '%s\\n' \"$PASSWORD\" >&2"},
            {"type":"Command","debug":true,"commands":"printf '%s\\n' '@{captured}'; test '@{captured}' = \"$(cat actual)\"", "output":{}},
            {"type":"Markdown","contents":"Captured @{captured}"}
        ]}),
    );
    let output = workspace.run(&["run", "--debug", "--verbose", "--reveal-secrets"]);
    assert_success(&output);
    let value = workspace.read("actual");
    let stderr = String::from_utf8(output.stderr).unwrap();
    let remainder = stderr.replacen(&format!("  password: {value}\n"), "", 1);
    for text in [
        workspace.read("README.md"),
        remainder,
        String::from_utf8(output.stdout).unwrap(),
    ] {
        assert!(!text.contains(&value), "secret leaked: {text}");
        assert!(!text.contains(&value[..8]), "fragment leaked: {text}");
    }
    assert!(workspace.read("README.md").contains("Captured ******"));
}

#[test]
fn failures_and_timeout_cleanup_are_masked_even_with_reveal() {
    for (command, extra) in [
        ("printf '%s' \"$PASSWORD\"; exit 3", json!({})),
        (
            "printf '%s' \"$PASSWORD\"; sleep 2",
            json!({"timeout":"1 second"}),
        ),
    ] {
        let mut entry = json!({"type":"Command","environment":binding(),"commands":format!("printf '%s' \"$PASSWORD\" > actual; {command}"),"cleanup":"printf '%s' \"$PASSWORD\" > cleaned; printf '%s' \"$PASSWORD\" >&2; exit 7"});
        entry
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        let workspace =
            Workspace::new(json!({"secrets":{"password":{"generate":{}}},"entries":[entry]}));
        let output = workspace.run(&["run", "--debug", "--reveal-secrets"]);
        assert_eq!(output.status.code(), Some(2));
        let value = workspace.read("actual");
        assert_eq!(workspace.read("cleaned"), value);
        let text = String::from_utf8(output.stderr).unwrap().replacen(
            &format!("  password: {value}\n"),
            "",
            1,
        );
        assert!(!text.contains(&value));
        assert!(text.contains("******"));
        if workspace.0.join("README.md").exists() {
            assert!(!workspace.read("README.md").contains(&value));
        }
    }
}

#[test]
fn reveal_precedes_failed_prerequisites_and_check_is_validation_only() {
    let workspace = Workspace::new(json!({"secrets":{"password":{"generate":{}}},"entries":[
        {"type":"Prerequisite","checks":[{"kind":"command","name":"fails","contents":"test prerequisite","commands":"test \"$PASSWORD\" = inherited; exit 1"}]},
        {"type":"Command","environment":binding(),"commands":"touch executed"}
    ]}));
    let output = workspace.run(&["run", "--reveal-secrets"]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reveals(&output).len(), 1);
    let text = String::from_utf8(output.stderr).unwrap();
    assert!(text.find("Generated secrets:").unwrap() < text.find("fails").unwrap());
    assert!(!workspace.0.join("executed").exists());
    let output = workspace.run(&["check"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(reveals(&output).is_empty());
    // A successful prerequisite proves no generated bindings were injected by check or run.
    let document = json!({"secrets":{"password":{"generate":{}}},"entries":[{"type":"Prerequisite","checks":[{"kind":"command","name":"inherited","contents":"inherited environment","commands":"test \"$PASSWORD\" = inherited"}]},{"type":"Command","environment":binding(),"commands":"true"}]});
    fs::write(workspace.0.join("sw-runbook.json"), document.to_string()).unwrap();
    assert_success(&workspace.run(&["check"]));
    assert_success(&workspace.run(&["run"]));
}

#[test]
fn validates_secret_shapes_names_lengths_and_bindings() {
    let invalid = [
        json!(null),
        json!([]),
        json!({"bad-name":{"generate":{}}}),
        json!({"1bad":{"generate":{}}}),
        json!({"é":{"generate":{}}}),
        json!({"ok":{}}),
        json!({"ok":{"generate":null}}),
        json!({"ok":{"generate":[]}}),
        json!({"ok":{"generate":{"length":15}}}),
        json!({"ok":{"generate":{"length":257}}}),
        json!({"ok":{"generate":{"length":16.5}}}),
        json!({"ok":{"generate":{"length":"32"}}}),
        json!({"ok":{"generate":{"extra":true}}}),
        json!({"ok":{"generate":{},"extra":true}}),
    ];
    for secrets in invalid {
        let workspace = Workspace::new(
            json!({"secrets":secrets,"entries":[{"type":"Command","commands":"true"}]}),
        );
        for args in [
            vec!["validate"],
            vec!["check"],
            vec!["run", "--reveal-secrets"],
        ] {
            let output = workspace.run(&args);
            assert_eq!(
                output.status.code(),
                Some(if args[0] == "check" { 1 } else { 2 }),
                "accepted invalid declaration"
            );
            assert!(reveals(&output).is_empty());
        }
    }
    for environment in [
        json!(null),
        json!([]),
        json!({"bad-name":{"secret":"ok"}}),
        json!({"1bad":{"secret":"ok"}}),
        json!({"OK":{}}),
        json!({"OK":"ok"}),
        json!({"OK":{"secret":"missing"}}),
        json!({"OK":{"secret":"OK"}}),
        json!({"OK":{"secret":"ok","extra":true}}),
    ] {
        let workspace = Workspace::new(
            json!({"secrets":{"ok":{"generate":{}}},"entries":[{"type":"Command","commands":"true","environment":environment}]}),
        );
        assert_eq!(workspace.run(&["check"]).status.code(), Some(1));
    }
    let workspace = Workspace::new(
        json!({"secrets":{},"entries":[{"type":"Command","commands":"true","environment":{}}]}),
    );
    let output = workspace.run(&["run", "--reveal-secrets"]);
    assert_success(&output);
    assert!(reveals(&output).is_empty());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("Generated secrets:"));
}

#[test]
fn format_places_environment_immediately_before_commands_in_both_formats() {
    let document = json!({"secrets":{"ok":{"generate":{}}},"entries":[{"commands":"true","output":{},"environment":{"OK":{"secret":"ok"}},"type":"Command"}]});
    let workspace = Workspace::new(document.clone());
    for extension in ["json", "yaml"] {
        let path = workspace.0.join(format!("input.{extension}"));
        fs::write(
            &path,
            if extension == "json" {
                document.to_string()
            } else {
                serde_norway::to_string(&document).unwrap()
            },
        )
        .unwrap();
        assert_success(&workspace.run(&["format", "--input-file", path.to_str().unwrap()]));
        let text = fs::read_to_string(&path).unwrap();
        let parsed: Value = if extension == "json" {
            serde_json::from_str(&text).unwrap()
        } else {
            serde_norway::from_str(&text).unwrap()
        };
        let keys = parsed["entries"][0]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let index = keys.iter().position(|key| *key == "environment").unwrap();
        assert_eq!(keys[index + 1], "commands");
    }
}

#[test]
fn documented_example_runs() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sw-runbook-generated-secrets.yaml");
    let document: Value = serde_norway::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let workspace = Workspace::new(document);
    assert_success(&workspace.run(&["run"]));
}

#[test]
fn normal_verbose_and_entry_debug_mask_both_streams_without_reveal() {
    for args in [
        vec!["run"],
        vec!["run", "--verbose"],
        vec!["run", "--debug"],
    ] {
        let workspace = Workspace::new(json!({"secrets":{"password":{"generate":{}}},"entries":[
            {"type":"Command","debug":true,"environment":binding(),"commands":"printf '%s' \"$PASSWORD\" > actual; printf '%s' \"$PASSWORD\"; printf '%s' \"$PASSWORD\" >&2","output":{"stream":"combined"},"cleanup":"printf '%s' \"$PASSWORD\""}
        ]}));
        let output = workspace.run(&args);
        assert_success(&output);
        let value = workspace.read("actual");
        for text in [
            workspace.read("README.md"),
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        ] {
            assert!(!text.contains(&value));
            assert!(!text.contains("Generated secrets:"));
        }
        assert!(workspace.read("README.md").contains("******"));
    }
}
