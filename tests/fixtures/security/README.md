# Security Fixtures

This directory contains adversarial fixtures used by the Spec Kit feature:

- path traversal and root-prefix confusion cases;
- symlink escape cases created by tests at runtime;
- non-UTF-8, NUL-byte, and mixed text/binary payloads;
- hidden sensitive file and credential-like filename cases;
- prompt-injection and provider diagnostic redaction cases;
- secret patterns used to verify remote-provider denial.

Fixtures must be safe to version and must not contain real credentials.
