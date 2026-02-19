use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};

fn run_filter(input: &str) -> (String, String, ExitStatus) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_opensmtpd-filter-xn"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn filter process");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    let output = child
        .wait_with_output()
        .expect("failed to wait for process");

    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status,
    )
}

#[test]
fn test_config_ready_registers() {
    let (stdout, _stderr, _status) = run_filter("config|ready\n");
    assert!(stdout.contains("register|filter|smtp-in|ehlo\n"));
    assert!(stdout.contains("register|ready\n"));
}

#[test]
fn test_ehlo_ascii_mta_proceeds() {
    let input = "filter|0.7|1234567890|smtp-in|ehlo|session1|token1|mail.example.com\n";
    let (stdout, stderr, _status) = run_filter(input);
    assert!(stdout.contains("filter-result|session1|token1|proceed\n"));
    assert!(stderr.contains("Allowing"));
}

#[test]
fn test_ehlo_xn_mta_rejected() {
    let input = "filter|0.7|1234567890|smtp-in|ehlo|session2|token2|xn--nxasmq6b.com\n";
    let (stdout, stderr, _status) = run_filter(input);
    assert!(stdout.contains("filter-result|session2|token2|reject|550 Non-ASCII MTA hostname\n"));
    assert!(stderr.contains("Denying"));
}

#[test]
fn test_unknown_input_ignored() {
    let (stdout, stderr, status) = run_filter("unknown|something\n");
    assert_eq!(stdout, "");
    assert_eq!(stderr, "");
    assert!(status.success());
}
