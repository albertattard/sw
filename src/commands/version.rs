use std::process::ExitCode;

pub const DISPLAY_VERSION: &str = env!("SW_CLI_VERSION");

pub fn run() -> ExitCode {
    println!("sw {DISPLAY_VERSION}");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn compose_version_uses_package_version_when_git_metadata_is_missing() {
        assert_eq!(compose_version("0.1.0", None, false), "0.1.0");
    }

    #[test]
    fn compose_version_includes_git_commit_when_available() {
        assert_eq!(
            compose_version("0.1.0", Some("abc1234"), false),
            "0.1.0 (abc1234)"
        );
    }

    #[test]
    fn compose_version_appends_dirty_marker_when_requested() {
        assert_eq!(
            compose_version("0.1.0", Some("abc1234"), true),
            "0.1.0 (abc1234-dirty)"
        );
    }
}
