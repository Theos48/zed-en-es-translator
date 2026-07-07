use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use translator_core::{translate_text, ErrorCode, LibreTranslateProvider, ProviderTarget};

#[test]
fn maps_status_rejection_to_provider_failed() {
    let provider = provider_for_response(429, r#"{"error":"quota exceeded"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("status failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_unsupported_language_pair_rejection_to_provider_failed() {
    let provider = provider_for_response(400, r#"{"error":"unsupported language pair"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("language failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_malformed_response_to_provider_failed() {
    let provider = provider_for_response(200, r#"{"unexpected":"shape"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("malformed failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_empty_response_text_to_provider_failed() {
    let provider = provider_for_response(200, r#"{"translatedText":""}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("empty failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

fn provider_for_response(status: u16, response_body: &'static str) -> LibreTranslateProvider {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
    let url = format!("http://{}", listener.local_addr().expect("local addr"));
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("set timeout");
        let mut buffer = [0_u8; 4096];
        let _ = stream.read(&mut buffer).expect("read request");
        let status_text = if status == 200 { "OK" } else { "ERROR" };
        write!(
            stream,
            "HTTP/1.1 {status} {status_text}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
            response_body.len()
        )
        .expect("write response");
    });
    LibreTranslateProvider::new(ProviderTarget::parse(&url, false).expect("target"), None)
}
