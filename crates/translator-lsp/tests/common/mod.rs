#![allow(dead_code)]

use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use lsp_server::{
    Connection, Message, Notification, Request, RequestId, Response, ResponseError, ResponseKind,
};
use lsp_types::{Position, Range};
use serde_json::{json, Value};
use translator_core::{MockProvider, Provider};
use translator_lsp::serve;

pub trait ResponseExt {
    fn result(&self) -> Option<&Value>;
    fn error(&self) -> Option<&ResponseError>;
}

impl ResponseExt for Response {
    fn result(&self) -> Option<&Value> {
        match &self.response_kind {
            ResponseKind::Ok { result } => Some(result),
            ResponseKind::Err { .. } => None,
        }
    }

    fn error(&self) -> Option<&ResponseError> {
        match &self.response_kind {
            ResponseKind::Ok { .. } => None,
            ResponseKind::Err { error } => Some(error),
        }
    }
}

pub struct TestClient {
    connection: Connection,
    server: Option<JoinHandle<Result<(), translator_lsp::ServerError>>>,
    next_id: i32,
}

impl TestClient {
    pub fn new() -> Self {
        Self::with_provider(PathBuf::from("/workspace"), MockProvider::new())
    }

    pub fn with_workspace(workspace: PathBuf) -> Self {
        Self::with_provider(workspace, MockProvider::new())
    }

    pub fn with_provider<P: Provider + Send + 'static>(workspace: PathBuf, provider: P) -> Self {
        let (server_connection, connection) = Connection::memory();
        let server = thread::spawn(move || serve(server_connection, workspace, provider));
        let mut client = Self {
            connection,
            server: Some(server),
            next_id: 1,
        };
        let response = client.request("initialize", json!({"capabilities": {}}));
        assert!(response.error().is_none());
        client.notify("initialized", json!({}));
        client
    }

    pub fn open(&self, uri: &str, version: i32, language_id: &str, text: &str) {
        self.notify(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": uri,
                    "version": version,
                    "languageId": language_id,
                    "text": text
                }
            }),
        );
    }

    pub fn change(&self, uri: &str, version: i32, text: &str) {
        self.notify(
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": version},
                "contentChanges": [{"text": text}]
            }),
        );
    }

    pub fn request(&mut self, method: &str, params: Value) -> Response {
        self.request_with_messages(method, params).0
    }

    pub fn begin_request(&mut self, method: &str, params: Value) -> RequestId {
        let id = RequestId::from(self.next_id);
        self.next_id += 1;
        self.connection
            .sender
            .send(Message::Request(Request {
                id: id.clone(),
                method: method.to_string(),
                params,
            }))
            .expect("request send");
        id
    }

    pub fn receive(&self) -> Message {
        self.connection.receiver.recv().expect("server message")
    }

    pub fn respond(&self, response: Response) {
        self.connection
            .sender
            .send(Message::Response(response))
            .expect("response send");
    }

    pub fn receive_response(&self, id: &RequestId) -> (Response, Vec<Message>) {
        let mut preceding = Vec::new();
        loop {
            match self.receive() {
                Message::Response(response) if &response.id == id => return (response, preceding),
                other => preceding.push(other),
            }
        }
    }

    pub fn request_with_messages(
        &mut self,
        method: &str,
        params: Value,
    ) -> (Response, Vec<Message>) {
        let id = self.begin_request(method, params);
        self.receive_response(&id)
    }

    pub fn notify(&self, method: &str, params: Value) {
        self.connection
            .sender
            .send(Message::Notification(Notification {
                method: method.to_string(),
                params,
            }))
            .expect("notification send");
    }

    pub fn shutdown(mut self) {
        let response = self.request("shutdown", Value::Null);
        assert!(response.error().is_none());
        self.notify("exit", Value::Null);
        assert!(self
            .server
            .take()
            .expect("server thread")
            .join()
            .expect("server join")
            .is_ok());
    }
}

pub fn range(start: u32, end: u32) -> Range {
    Range::new(Position::new(0, start), Position::new(0, end))
}

pub fn code_action_params(uri: &str, range: Range) -> Value {
    json!({
        "textDocument": {"uri": uri},
        "range": range,
        "context": {"diagnostics": []}
    })
}

pub fn file_uri(path: &std::path::Path) -> String {
    url::Url::from_file_path(path)
        .expect("absolute file path")
        .to_string()
}
