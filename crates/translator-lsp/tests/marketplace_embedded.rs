mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;

use serde_json::{json, Value};
use translator_core::{EmbeddedProcessProvider, EmbeddedProcessRunner, EmbeddedRunnerLimits};

use common::{file_uri, range, ResponseExt as _, TestClient};

#[test]
fn embedded_marketplace_provider_is_offline_and_previews_without_mutation() {
    let workspace = unique_root("preview");
    let runner_root = unique_root("runner");
    let runner_path = runner_root.join("translator-embedded-runtime");
    fs::create_dir_all(&runner_root).expect("runner root");
    fs::write(
        &runner_path,
        "#!/bin/sh\nwhile IFS= read -r _; do :; done\nprintf '%s' '{\"wire_version\":1,\"translations\":[\"Traduccion local controlada.\"]}'\n",
    )
    .expect("runner fixture");
    fs::set_permissions(&runner_path, fs::Permissions::from_mode(0o700))
        .expect("runner permissions");
    let runner = EmbeddedProcessRunner::from_verified_paths(
        runner_path,
        runner_root.clone(),
        EmbeddedRunnerLimits::for_tests(Duration::from_secs(1)),
    )
    .expect("controlled runner");
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    fs::create_dir_all(&workspace).expect("workspace");
    let document = workspace.join("source.md");
    let disk_source = b"Read this source.";
    fs::write(&document, disk_source).expect("source fixture");
    let uri = file_uri(&document);
    let mut client = TestClient::with_provider(workspace.clone(), provider);
    client.open(&uri, 1, "markdown", "Read this source.");

    let execute = client.request(
        "workspace/executeCommand",
        json!({
            "command":"en-es-translator.translate",
            "arguments":[{"uri":uri,"version":1,"range":range(0,0),"input_kind":"markdown"}]
        }),
    );
    assert_eq!(execute.result(), Some(&Value::Null));
    let hover = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert_eq!(
        hover.result().expect("hover")["contents"]["value"],
        "Traduccion local controlada."
    );
    assert_eq!(
        fs::read(&document).expect("source after preview"),
        disk_source
    );
    client.shutdown();
    fs::remove_dir_all(workspace).expect("remove workspace");
    fs::remove_dir_all(runner_root).expect("remove runner root");
}

fn unique_root(case: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "marketplace-embedded-{case}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ))
}
