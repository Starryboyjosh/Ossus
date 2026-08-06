#![allow(
    clippy::expect_used,
    reason = "CLI integration tests use expect for deterministic fixture setup"
)]

use assert_cmd::Command;

fn cli_output(arguments: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ossus"))
        .args(arguments)
        .output()
        .expect("ossus binary should be runnable");

    let status = output
        .status
        .code()
        .map_or_else(|| "signal".to_owned(), |code| code.to_string());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    format!("status: {status}\nstdout:\n{stdout}stderr:\n{stderr}")
}

#[test]
fn version_output() {
    insta::assert_snapshot!(cli_output(&["--version"]));
}

#[test]
fn root_help_output() {
    insta::assert_snapshot!(cli_output(&["--help"]));
}

macro_rules! placeholder_snapshot_tests {
    ($($name:ident => $command:literal),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                insta::assert_snapshot!(cli_output(&[$command, "--help"]));
            }
        )+
    };
}

placeholder_snapshot_tests! {
    placeholder_init => "init",
    placeholder_config => "config",
    placeholder_registry => "registry",
    placeholder_search => "search",
    placeholder_show => "show",
    placeholder_scan => "scan",
    placeholder_resolve => "resolve",
    placeholder_explain => "explain",
    placeholder_activate => "activate",
    placeholder_deactivate => "deactivate",
    placeholder_lock => "lock",
    placeholder_doctor => "doctor",
    placeholder_eval => "eval",
    placeholder_audit => "audit",
    placeholder_research => "research",
}
