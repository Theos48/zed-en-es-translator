use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use translator_core::{translate_file, translate_text, LibreTranslateProvider, ProviderTarget};

#[test]
fn translates_direct_text_with_local_provider() {
    let server = one_response_server(r#"{"translatedText":"Lee la documentacion."}"#);
    let provider =
        LibreTranslateProvider::new(ProviderTarget::parse(&server, false).expect("target"), None);

    let translated = translate_text("Read the docs.", &provider).expect("translation");

    assert_eq!(translated.translated_text, "Lee la documentacion.");
}

#[test]
fn translates_allowed_file_without_mutating_source() {
    let server = response_server(8, r#"{"translatedText":"Abre el archivo."}"#);
    let provider =
        LibreTranslateProvider::new(ProviderTarget::parse(&server, false).expect("target"), None);
    let workspace = temp_workspace("local-provider-file");
    let file = workspace.join("notes.md");
    let original = "# Notes\n\nOpen the file.\n\n```rust\nfn main() {}\n```\n";
    std::fs::write(&file, original).expect("write file");

    let translated = translate_file(
        "notes.md",
        workspace.to_str().expect("workspace"),
        &provider,
    )
    .expect("file translation");

    assert!(translated.translated_text.contains("Abre el archivo."));
    assert_eq!(std::fs::read_to_string(file).expect("read file"), original);
}

fn one_response_server(response_body: &'static str) -> String {
    response_server(1, response_body)
}

fn response_server(requests: usize, response_body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
    let url = format!("http://{}", listener.local_addr().expect("local addr"));
    thread::spawn(move || {
        for _ in 0..requests {
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
        }
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

fn temp_workspace(case: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_{case}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    std::fs::create_dir_all(&root).expect("temp root");
    root
}
