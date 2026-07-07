mod common;

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use translator_core::{ProviderConfiguration, ProviderSelection};
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn mcp_uses_configured_local_provider() {
    let url = response_server(r#"{"translatedText":"Salida MCP local."}"#);
    let config = ProviderConfiguration::from_values(Some("libretranslate"), Some(&url), None, None)
        .expect("provider config");
    let provider = ProviderSelection::from_configuration(config).expect("provider selection");
    let value = serde_json::to_value(
        TranslatorMcpServer::with_provider(provider)
            .translate_text(common::translate_text_params("Read the docs.")),
    )
    .expect("tool result json");

    assert_eq!(value["isError"], Value::Bool(false));
    assert_eq!(
        value["structuredContent"]["translated_text"],
        "Salida MCP local."
    );
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
