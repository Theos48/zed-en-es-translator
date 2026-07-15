use std::io::Write;
use std::process::{Command, Stdio};

use translator_core::{ENV_PROVIDER, ENV_PROVIDER_URL};

mod common;

#[test]
fn cli_uses_mock_provider_when_unconfigured() {
    let output = run_cli_with_env(request_json("Read the docs.", false), &[]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout utf-8"),
        r#"{"translated_text":"Lee la documentacion."}"#
    );
}

#[test]
fn cli_selects_local_libretranslate_provider_from_environment() {
    let url =
        common::operational_local_response_server(r#"{"translatedText":["Salida real local."]}"#);
    let output = run_cli_with_env(
        request_json("Read the docs.", false),
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout utf-8"),
        r#"{"translated_text":"Salida real local."}"#
    );
}

fn request_json(source_text: &str, remote_confirmed: bool) -> String {
    let suffix = if remote_confirmed {
        r#","remote_confirmed":true"#
    } else {
        ""
    };
    format!(
        r#"{{"source_text":"{source_text}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"{suffix}}}"#
    )
}

fn run_cli_with_env(input: String, env: &[(&str, &str)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_translator-cli"));
    command
        .env_remove(ENV_PROVIDER)
        .env_remove(ENV_PROVIDER_URL)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in env {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("spawn translator-cli");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait for CLI")
}
