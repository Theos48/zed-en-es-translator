use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use translator_core::{translate_text, ErrorCode, LibreTranslateProvider, ProviderTarget};

#[test]
fn maps_provider_timeout() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind slow server");
    let url = format!("http://{}", listener.local_addr().expect("local addr"));
    thread::spawn(move || {
        let (_stream, _) = listener.accept().expect("accept request");
        thread::sleep(Duration::from_millis(250));
    });
    let provider = LibreTranslateProvider::with_timeout(
        ProviderTarget::parse(&url, false).expect("target"),
        None,
        Duration::from_millis(50),
    );

    let err = translate_text("Read the docs.", &provider).expect_err("timeout");

    assert_eq!(err.code, ErrorCode::ProviderTimeout);
}
