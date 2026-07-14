"""Fixed loopback relay for an egress-isolated LibreTranslate container."""

from __future__ import annotations

import http.client
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


UPSTREAM_HOST = "libretranslate"
UPSTREAM_PORT = 5000
MAX_REQUEST_BYTES = 128 * 1024
MAX_RESPONSE_BYTES = 40 * 1024
IO_TIMEOUT_SECONDS = 15


class ProviderRelayHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    server_version = "ProviderLoopback/1"
    sys_version = ""

    def setup(self) -> None:
        super().setup()
        self.connection.settimeout(IO_TIMEOUT_SECONDS)

    def log_message(self, _format: str, *args: object) -> None:
        del args

    def do_GET(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        if self.path != "/health":
            self._send_empty(404)
            return
        self._relay("GET", "/health", None)

    def do_POST(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        if self.path != "/translate":
            self._send_empty(404)
            return
        if self.headers.get_content_type() != "application/json":
            self._send_empty(415)
            return
        try:
            content_length = int(self.headers.get("Content-Length", ""))
        except ValueError:
            self._send_empty(400)
            return
        if content_length < 1 or content_length > MAX_REQUEST_BYTES:
            self._send_empty(413)
            return
        body = self.rfile.read(content_length)
        if len(body) != content_length:
            self._send_empty(400)
            return
        self._relay("POST", "/translate", body)

    def _relay(self, method: str, path: str, body: bytes | None) -> None:
        connection = http.client.HTTPConnection(
            UPSTREAM_HOST,
            UPSTREAM_PORT,
            timeout=IO_TIMEOUT_SECONDS,
        )
        headers = {"Accept": "application/json"}
        if body is not None:
            headers["Content-Type"] = "application/json"
        try:
            connection.request(method, path, body=body, headers=headers)
            response = connection.getresponse()
            response_body = response.read(MAX_RESPONSE_BYTES + 1)
            if len(response_body) > MAX_RESPONSE_BYTES:
                self._send_empty(502)
                return
            self.send_response(response.status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(response_body)))
            self.end_headers()
            self.wfile.write(response_body)
        except (OSError, http.client.HTTPException):
            self._send_empty(502)
        finally:
            connection.close()

    def _send_empty(self, status: int) -> None:
        self.send_response(status)
        self.send_header("Content-Length", "0")
        self.end_headers()


def main() -> None:
    server = ThreadingHTTPServer(("0.0.0.0", 5000), ProviderRelayHandler)
    server.daemon_threads = True
    server.serve_forever()


if __name__ == "__main__":
    main()
