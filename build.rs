use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let package_version = env::var("CARGO_PKG_VERSION").expect("missing CARGO_PKG_VERSION");

    let git_commit = git_output(&manifest_dir, &["rev-parse", "--short", "HEAD"]);
    let dirty = git_is_dirty(&manifest_dir);
    let display_version = compose_version(&package_version, git_commit.as_deref(), dirty);

    println!(
        "cargo:rustc-env=SW_GIT_COMMIT={}",
        git_commit.as_deref().unwrap_or("")
    );
    println!(
        "cargo:rustc-env=SW_GIT_DIRTY={}",
        if dirty { "true" } else { "false" }
    );
    println!("cargo:rustc-env=SW_CLI_VERSION={display_version}");

    track_git_inputs(&manifest_dir);
}

fn compose_version(package_version: &str, git_commit: Option<&str>, dirty: bool) -> String {
    match git_commit {
        Some(commit) if !commit.is_empty() => {
            if dirty {
                format!("{package_version} ({commit}-dirty)")
            } else {
                format!("{package_version} ({commit})")
            }
        }
        _ => package_version.to_string(),
    }
}

fn git_is_dirty(manifest_dir: &Path) -> bool {
    let Some(output) = git_command(manifest_dir, &["status", "--porcelain"]) else {
        return false;
    };

    !output.trim().is_empty()
}

fn git_output(manifest_dir: &Path, args: &[&str]) -> Option<String> {
    let output = git_command(manifest_dir, args)?;
    let trimmed = output.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn git_command(manifest_dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(manifest_dir)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn track_git_inputs(manifest_dir: &Path) {
    let paths = [
        git_output(manifest_dir, &["rev-parse", "--git-path", "HEAD"]),
        git_output(manifest_dir, &["rev-parse", "--git-path", "index"]),
        git_output(manifest_dir, &["rev-parse", "--git-path", "packed-refs"]),
        git_output(manifest_dir, &["symbolic-ref", "-q", "HEAD"])
            .and_then(|head_ref| git_output(manifest_dir, &["rev-parse", "--git-path", &head_ref])),
    ];

    for path in paths.into_iter().flatten() {
        let resolved = if Path::new(&path).is_absolute() {
            PathBuf::from(&path)
        } else {
            manifest_dir.join(path)
        };

        if resolved.exists() {
            println!("cargo:rerun-if-changed={}", resolved.display());
        }
    }
}
