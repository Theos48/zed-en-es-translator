#![allow(dead_code)]

use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

pub fn response_server(response_body: &'static str) -> String {
    response_server_with_status(200, response_body)
}

pub fn operational_local_response_server(response_body: &'static str) -> String {
    operational_local_response_server_with_status(200, response_body)
}

pub fn operational_local_response_server_with_status(
    status: u16,
    response_body: &'static str,
) -> String {
    response_server_on_listener(
        TcpListener::bind("127.0.0.1:5000").expect("bind operational stub server"),
        status,
        response_body,
    )
}

pub fn response_server_with_status(status: u16, response_body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub server");
    response_server_on_listener(listener, status, response_body)
}

fn response_server_on_listener(
    listener: TcpListener,
    status: u16,
    response_body: &'static str,
) -> String {
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
