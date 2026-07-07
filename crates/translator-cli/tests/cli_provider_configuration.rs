use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use translator_core::{ENV_PROVIDER, ENV_PROVIDER_URL};

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
    let url = response_server(r#"{"translatedText":"Salida real local."}"#);
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

fn response_server(response_body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
    let url = format!("http://{}", listener.local_addr().expect("local addr"));
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("set timeout");
        read_http_request(&mut stream);
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
            response_body.len()
        )
        .expect("write response");
    });
    url
}

fn read_http_request(stream: &mut impl Read) {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let read = match stream.read(&mut buffer) {
            Ok(read) => read,
            Err(error) if error.kind() == ErrorKind::WouldBlock => break,
            Err(error) if error.kind() == ErrorKind::TimedOut => break,
            Err(error) => panic!("read request: {error}"),
        };
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
        if request_is_complete(&bytes) {
            break;
        }
    }
}

fn request_is_complete(bytes: &[u8]) -> bool {
    let request = String::from_utf8_lossy(bytes);
    let Some((headers, body)) = request.split_once("\r\n\r\n") else {
        return false;
    };
    if headers
        .lines()
        .any(|line| line.eq_ignore_ascii_case("Transfer-Encoding: chunked"))
    {
        return body.contains("\r\n0\r\n\r\n");
    }
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(usize::MAX);
    body.len() >= content_length
}
