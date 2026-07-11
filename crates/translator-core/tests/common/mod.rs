#![allow(dead_code)]

use std::fs;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static UNIQUE_SUFFIX_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_{}_{}_{}",
        name,
        std::process::id(),
        unique_suffix()
    ));
    fs::create_dir_all(&root).expect("temp root");
    root
}

pub fn write_file(path: &Path, content: impl AsRef<[u8]>) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir");
    }
    fs::write(path, content).expect("write file");
}

fn unique_suffix() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let counter = UNIQUE_SUFFIX_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{nanos}_{counter}")
}

pub struct StubHttpServer {
    url: String,
    bodies: Arc<Mutex<Vec<String>>>,
}

impl StubHttpServer {
    pub fn new(response_body: impl Into<String>) -> Self {
        Self::with_status_and_requests(200, 1, response_body)
    }

    pub fn with_requests(requests: usize, response_body: impl Into<String>) -> Self {
        Self::with_status_and_requests(200, requests, response_body)
    }

    pub fn with_status(status: u16, response_body: impl Into<String>) -> Self {
        Self::with_status_and_requests(status, 1, response_body)
    }

    pub fn with_status_and_requests(
        status: u16,
        requests: usize,
        response_body: impl Into<String>,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
        let url = format!("http://{}", listener.local_addr().expect("local addr"));
        let bodies = Arc::new(Mutex::new(Vec::new()));
        let thread_bodies = Arc::clone(&bodies);
        let response_body = response_body.into();

        thread::spawn(move || {
            for _ in 0..requests {
                let (mut stream, _) = listener.accept().expect("accept request");
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .expect("set timeout");
                let request = read_http_request(&mut stream);
                let body = http_body(&request);
                thread_bodies.lock().expect("bodies lock").push(body);
                write_response(&mut stream, status, &response_body);
            }
        });

        Self { url, bodies }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn first_body(&self) -> String {
        self.bodies(1).remove(0)
    }

    pub fn bodies(&self, expected: usize) -> Vec<String> {
        for _ in 0..20 {
            let bodies = self.bodies.lock().expect("bodies lock").clone();
            if bodies.len() >= expected {
                return bodies;
            }
            thread::sleep(Duration::from_millis(10));
        }
        panic!("stub server did not receive {expected} request(s)");
    }
}

fn read_http_request(stream: &mut impl Read) -> String {
    const MAX_REQUEST_BYTES: usize = 1024 * 1024;

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
        if read > MAX_REQUEST_BYTES.saturating_sub(bytes.len()) {
            panic!("stub request exceeds the maximum size");
        }
        bytes.extend_from_slice(&buffer[..read]);
        if request_is_complete(&bytes) {
            break;
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn write_response(stream: &mut impl Write, status: u16, body: &str) {
    let status_text = if status == 200 { "OK" } else { "ERROR" };
    write!(
        stream,
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("write response");
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

fn http_body(request: &str) -> String {
    let Some((headers, body)) = request.split_once("\r\n\r\n") else {
        return String::new();
    };
    if headers
        .lines()
        .any(|line| line.eq_ignore_ascii_case("Transfer-Encoding: chunked"))
    {
        decode_chunked_body(body)
    } else {
        body.to_string()
    }
}

fn decode_chunked_body(body: &str) -> String {
    let mut decoded = String::new();
    let mut rest = body;
    loop {
        let Some((size_hex, after_size)) = rest.split_once("\r\n") else {
            return decoded;
        };
        let Ok(size) = usize::from_str_radix(size_hex.trim(), 16) else {
            return decoded;
        };
        if size == 0 {
            return decoded;
        }
        if after_size.len() < size + 2 {
            return decoded;
        }
        decoded.push_str(&after_size[..size]);
        rest = &after_size[size + 2..];
    }
}
