use std::path::PathBuf;
use std::thread;

use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
use serde_json::json;
use translator_core::MockProvider;
use translator_lsp::{serve, state::ProviderDescriptor};

#[test]
fn advertises_minimal_capabilities_and_handles_shutdown() {
    let (server, client) = Connection::memory();
    let server_thread = thread::spawn(|| {
        serve(
            server,
            PathBuf::from("/workspace"),
            MockProvider::new(),
            ProviderDescriptor::offline(),
        )
    });

    client
        .sender
        .send(Message::Request(Request {
            id: 1.into(),
            method: "initialize".to_string(),
            params: json!({"capabilities": {}}),
        }))
        .expect("initialize send");
    let response = receive_response(&client);
    let capabilities = response
        .result
        .expect("initialize result")
        .get("capabilities")
        .cloned()
        .expect("capabilities");
    assert_eq!(capabilities["textDocumentSync"]["openClose"], true);
    assert_eq!(capabilities["textDocumentSync"]["change"], 1);
    assert_eq!(capabilities["hoverProvider"], true);
    assert_eq!(
        capabilities["executeCommandProvider"]["commands"],
        json!(["en-es-translator.translate"])
    );

    client
        .sender
        .send(Message::Notification(Notification {
            method: "initialized".to_string(),
            params: json!({}),
        }))
        .expect("initialized send");

    client
        .sender
        .send(Message::Request(Request {
            id: 2.into(),
            method: "unknown/private-method".to_string(),
            params: json!({"source": "SOURCE_SECRET_123"}),
        }))
        .expect("unknown send");
    let unknown = receive_response(&client);
    let error = unknown.error.expect("unknown method error");
    assert_eq!(error.code, ErrorCode::MethodNotFound as i32);
    assert!(!error.message.contains("SOURCE_SECRET_123"));

    client
        .sender
        .send(Message::Request(Request {
            id: 3.into(),
            method: "shutdown".to_string(),
            params: serde_json::Value::Null,
        }))
        .expect("shutdown send");
    let shutdown = receive_response(&client);
    assert!(shutdown.error.is_none());
    client
        .sender
        .send(Message::Notification(Notification {
            method: "exit".to_string(),
            params: serde_json::Value::Null,
        }))
        .expect("exit send");

    assert!(server_thread.join().expect("server join").is_ok());
}

fn receive_response(connection: &Connection) -> Response {
    match connection.receiver.recv().expect("server message") {
        Message::Response(response) => response,
        other => panic!("expected response, got {other:?}"),
    }
}
