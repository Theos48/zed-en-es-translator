use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use translator_core::{ENV_PROVIDER, ENV_PROVIDER_URL};

#[test]
fn cli_redacts_provider_failure_stdout_and_stderr() {
    let url = response_server(
        429,
        r#"{"error":"source_text=Read the docs. Authorization: Bearer fake_token"}"#,
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .env(ENV_PROVIDER, "libretranslate")
        .env(ENV_PROVIDER_URL, url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn translator-cli");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(br#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#)
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

    assert!(!output.status.success());
    assert!(stdout.contains(r#""code":"PROVIDER_FAILED""#));
    assert!(stderr.contains("PROVIDER_FAILED"));
    assert!(!stdout.contains("fake_token"));
    assert!(!stderr.contains("fake_token"));
    assert!(!stdout.contains("Read the docs."));
    assert!(!stderr.contains("Read the docs."));
}

fn response_server(status: u16, response_body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
    let url = format!("http://{}", listener.local_addr().expect("local addr"));
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("set timeout");
        read_http_request(&mut stream);
        let status_text = if status == 200 { "OK" } else { "ERROR" };
        write!(
            stream,
            "HTTP/1.1 {status} {status_text}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
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
