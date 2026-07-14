//! LSP request and notification handling.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use lsp_server::{
    Connection, ErrorCode as RpcErrorCode, Message, Notification, Request, Response, ResponseKind,
};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, Command, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, Hover, HoverContents,
    HoverParams, HoverProviderCapability, MarkupContent, MarkupKind, MessageActionItem,
    MessageType, Position, ServerCapabilities, ShowMessageParams, ShowMessageRequestParams,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
};
use translator_core::{
    translate_document_snapshot_with_confirmation, translate_selection_with_confirmation,
    ErrorCode, InputKind, Provider, TranslateFailure,
};

use crate::selection::{
    file_path_from_uri, full_document_range, range_to_offsets, TranslationTarget,
};
use crate::state::{DocumentStore, ProviderDescriptor, ProviderLocalityLabel, TranslationPreview};

pub const TRANSLATE_COMMAND: &str = "en-es-translator.translate";
const CONFIRM_ACTION: &str = "Send this request";
static CONFIRMATION_ID: AtomicU64 = AtomicU64::new(1);

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                ..TextDocumentSyncOptions::default()
            },
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            code_action_kinds: Some(vec![CodeActionKind::REFACTOR]),
            ..CodeActionOptions::default()
        })),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![TRANSLATE_COMMAND.to_string()],
            ..ExecuteCommandOptions::default()
        }),
        ..ServerCapabilities::default()
    }
}

pub fn serve<P: Provider>(
    connection: Connection,
    workspace_root: std::path::PathBuf,
    provider: P,
    provider_descriptor: ProviderDescriptor,
) -> Result<(), ServerError> {
    let capabilities = serde_json::to_value(server_capabilities()).map_err(|_| ServerError)?;
    connection
        .initialize(capabilities)
        .map_err(|_| ServerError)?;

    let mut documents = DocumentStore::default();
    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                if connection
                    .handle_shutdown(&request)
                    .map_err(|_| ServerError)?
                {
                    return Ok(());
                }
                handle_request(
                    &connection,
                    request,
                    &workspace_root,
                    &mut documents,
                    &provider,
                    provider_descriptor,
                );
            }
            Message::Notification(notification) => {
                handle_notification(notification, &mut documents);
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}

fn handle_request<P: Provider>(
    connection: &Connection,
    request: Request,
    workspace_root: &std::path::Path,
    documents: &mut DocumentStore,
    provider: &P,
    descriptor: ProviderDescriptor,
) {
    let response = match request.method.as_str() {
        "textDocument/codeAction" => code_action_response(request, documents, descriptor),
        "workspace/executeCommand" => execute_command_response(
            connection,
            request,
            documents,
            workspace_root,
            provider,
            descriptor,
        ),
        "textDocument/hover" => hover_response(request, documents),
        _ => Response::new_err(
            request.id,
            RpcErrorCode::MethodNotFound as i32,
            "Method not supported.".to_string(),
        ),
    };
    let _ = connection.sender.send(Message::Response(response));
}

fn code_action_response(
    request: Request,
    documents: &DocumentStore,
    descriptor: ProviderDescriptor,
) -> Response {
    let id = request.id;
    let Ok(params) = serde_json::from_value::<CodeActionParams>(request.params) else {
        return invalid_params(id);
    };
    let Some(snapshot) = documents.get(&params.text_document.uri) else {
        return Response::new_ok(id, Vec::<CodeActionOrCommand>::new());
    };
    if range_to_offsets(snapshot.text(), params.range).is_err() {
        return Response::new_ok(id, Vec::<CodeActionOrCommand>::new());
    }

    let title = descriptor.action_title().to_string();
    let target = TranslationTarget::new(
        params.text_document.uri,
        snapshot.version(),
        params.range,
        snapshot.input_kind(),
    );
    let Ok(argument) = serde_json::to_value(target) else {
        return internal_error(id);
    };
    let action = CodeAction {
        title: title.clone(),
        kind: Some(CodeActionKind::REFACTOR),
        command: Some(Command {
            title,
            command: TRANSLATE_COMMAND.to_string(),
            arguments: Some(vec![argument]),
        }),
        ..CodeAction::default()
    };
    Response::new_ok(id, vec![CodeActionOrCommand::CodeAction(action)])
}

fn execute_command_response<P: Provider>(
    connection: &Connection,
    request: Request,
    documents: &mut DocumentStore,
    workspace_root: &std::path::Path,
    provider: &P,
    descriptor: ProviderDescriptor,
) -> Response {
    let id = request.id;
    let Ok(params) = serde_json::from_value::<ExecuteCommandParams>(request.params) else {
        return invalid_params(id);
    };
    if params.command != TRANSLATE_COMMAND || params.arguments.len() != 1 {
        return invalid_params(id);
    }
    let Ok(target) = serde_json::from_value::<TranslationTarget>(params.arguments[0].clone())
    else {
        return invalid_params(id);
    };
    let Some(snapshot) = documents.get(&target.uri).cloned() else {
        return invalid_params(id);
    };
    let Ok(input_kind) = target.parsed_input_kind() else {
        return invalid_params(id);
    };
    if target.version != snapshot.version() || input_kind != snapshot.input_kind() {
        return invalid_params(id);
    }
    let Ok(selection) = range_to_offsets(snapshot.text(), target.range) else {
        return invalid_params(id);
    };
    let remote_confirmed = if descriptor.locality() == ProviderLocalityLabel::Remote {
        if !descriptor.allow_remote() {
            return translation_error(
                id,
                TranslateFailure::new(
                    ErrorCode::ProviderNotConfigured,
                    "The provider is not configured for this request.",
                ),
            );
        }
        if let Err(failure) = confirm_remote_request(connection, documents, &target) {
            return translation_error(id, failure);
        }
        let Some(current) = documents.get(&target.uri) else {
            return invalid_params(id);
        };
        if current.version() != target.version {
            return invalid_params(id);
        }
        true
    } else {
        false
    };
    let translation = if selection.is_empty() {
        let Ok(file_path) = file_path_from_uri(&target.uri) else {
            return invalid_params(id);
        };
        let (Some(file_path), Some(workspace_root)) = (file_path.to_str(), workspace_root.to_str())
        else {
            return invalid_params(id);
        };
        translate_document_snapshot_with_confirmation(
            file_path,
            workspace_root,
            snapshot.text(),
            provider,
            remote_confirmed,
        )
    } else {
        translate_selection_with_confirmation(
            snapshot.text(),
            input_kind,
            selection,
            provider,
            remote_confirmed,
        )
    };

    match translation {
        Ok(success) => {
            let preview_range = if target.range.start == target.range.end {
                full_document_range(snapshot.text())
            } else {
                target.range
            };
            documents.set_preview(TranslationPreview::new(
                target.uri,
                target.version,
                preview_range,
                success.translated_text,
                input_kind,
            ));
            send_safe_message(
                connection,
                MessageType::INFO,
                "Translation preview ready. Hover the source range to read it.",
            );
            Response::new_ok(id, ())
        }
        Err(failure) => {
            send_safe_message(
                connection,
                MessageType::ERROR,
                &format!("Translation failed: {}.", failure.code.as_str()),
            );
            translation_error(id, failure)
        }
    }
}

fn confirm_remote_request(
    connection: &Connection,
    documents: &mut DocumentStore,
    target: &TranslationTarget,
) -> Result<(), TranslateFailure> {
    let confirmation_id = lsp_server::RequestId::from(format!(
        "en-es-translator-confirmation-{}",
        CONFIRMATION_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let params = ShowMessageRequestParams {
        typ: MessageType::WARNING,
        message: "Remote translation will send permitted selected or document content outside this machine. Continue for this request?".to_string(),
        actions: Some(vec![MessageActionItem {
            title: CONFIRM_ACTION.to_string(),
            properties: HashMap::new(),
        }]),
    };
    let params = serde_json::to_value(params).map_err(|_| internal_failure())?;
    connection
        .sender
        .send(Message::Request(Request {
            id: confirmation_id.clone(),
            method: "window/showMessageRequest".to_string(),
            params,
        }))
        .map_err(|_| internal_failure())?;

    let mut target_changed = false;
    loop {
        let message = connection
            .receiver
            .recv_timeout(Duration::from_secs(15))
            .map_err(|_| confirmation_required())?;
        match message {
            Message::Response(response) if response.id == confirmation_id => {
                let confirmed = match response.response_kind {
                    ResponseKind::Ok { result } => {
                        serde_json::from_value::<MessageActionItem>(result)
                            .is_ok_and(|action| action.title == CONFIRM_ACTION)
                    }
                    ResponseKind::Err { .. } => false,
                };
                if !confirmed {
                    return Err(confirmation_required());
                }
                if target_changed {
                    return Err(TranslateFailure::new(
                        ErrorCode::InvalidInput,
                        "The document changed during confirmation.",
                    ));
                }
                return Ok(());
            }
            Message::Response(_) => {}
            Message::Notification(notification) => {
                if notification.method == "$/cancelRequest" {
                    return Err(confirmation_required());
                }
                target_changed |= notification_targets_uri(&notification, &target.uri);
                handle_notification(notification, documents);
            }
            Message::Request(request) => {
                let response = Response::new_err(
                    request.id,
                    RpcErrorCode::RequestFailed as i32,
                    "Request unavailable during remote confirmation.".to_string(),
                );
                let _ = connection.sender.send(Message::Response(response));
            }
        }
    }
}

fn notification_targets_uri(notification: &Notification, uri: &lsp_types::Uri) -> bool {
    if !matches!(
        notification.method.as_str(),
        "textDocument/didChange" | "textDocument/didClose"
    ) {
        return false;
    }
    notification
        .params
        .get("textDocument")
        .and_then(|document| document.get("uri"))
        .and_then(serde_json::Value::as_str)
        .is_some_and(|candidate| candidate == uri.as_str())
}

fn confirmation_required() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::RemoteConfirmationRequired,
        "Remote provider confirmation is required for this request.",
    )
}

fn internal_failure() -> TranslateFailure {
    TranslateFailure::new(ErrorCode::InternalError, "An internal error occurred.")
}

fn hover_response(request: Request, documents: &DocumentStore) -> Response {
    let id = request.id;
    let Ok(params) = serde_json::from_value::<HoverParams>(request.params) else {
        return invalid_params(id);
    };
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let Some(snapshot) = documents.get(&uri) else {
        return Response::new_ok(id, Option::<Hover>::None);
    };
    let Some(preview) = documents.preview(&uri) else {
        return Response::new_ok(id, Option::<Hover>::None);
    };
    if preview.version() != snapshot.version() || !range_contains(preview.source_range(), position)
    {
        return Response::new_ok(id, Option::<Hover>::None);
    }

    let value = match preview.input_kind() {
        InputKind::Markdown => preview.translated_text().to_string(),
        InputKind::Text => escape_plain_text_for_markdown(preview.translated_text()),
    };
    Response::new_ok(
        id,
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(preview.source_range()),
        }),
    )
}

fn invalid_params(id: lsp_server::RequestId) -> Response {
    Response::new_err(
        id,
        RpcErrorCode::InvalidParams as i32,
        "INVALID_INPUT: Invalid input.".to_string(),
    )
}

fn internal_error(id: lsp_server::RequestId) -> Response {
    Response::new_err(
        id,
        RpcErrorCode::InternalError as i32,
        "INTERNAL_ERROR: An internal error occurred.".to_string(),
    )
}

fn translation_error(id: lsp_server::RequestId, failure: TranslateFailure) -> Response {
    Response::new_err(
        id,
        RpcErrorCode::RequestFailed as i32,
        format!("{}: {}", failure.code.as_str(), failure.message),
    )
}

fn send_safe_message(connection: &Connection, typ: MessageType, message: &str) {
    let Ok(params) = serde_json::to_value(ShowMessageParams {
        typ,
        message: message.to_string(),
    }) else {
        return;
    };
    let _ = connection.sender.send(Message::Notification(Notification {
        method: "window/showMessage".to_string(),
        params,
    }));
}

fn range_contains(range: lsp_types::Range, position: Position) -> bool {
    position_at_or_after(position, range.start) && position_before(position, range.end)
}

fn position_at_or_after(position: Position, boundary: Position) -> bool {
    position.line > boundary.line
        || (position.line == boundary.line && position.character >= boundary.character)
}

fn position_before(position: Position, boundary: Position) -> bool {
    position.line < boundary.line
        || (position.line == boundary.line && position.character < boundary.character)
}

fn escape_plain_text_for_markdown(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        if matches!(
            ch,
            '\\' | '`'
                | '*'
                | '_'
                | '{'
                | '}'
                | '['
                | ']'
                | '<'
                | '>'
                | '#'
                | '+'
                | '-'
                | '!'
                | '|'
        ) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

fn handle_notification(notification: Notification, documents: &mut DocumentStore) {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            if let Ok(params) =
                serde_json::from_value::<DidOpenTextDocumentParams>(notification.params)
            {
                let document = params.text_document;
                let _ = documents.open(
                    document.uri,
                    document.version,
                    &document.language_id,
                    document.text,
                );
            }
        }
        "textDocument/didChange" => {
            if let Ok(params) =
                serde_json::from_value::<DidChangeTextDocumentParams>(notification.params)
            {
                documents.invalidate_preview(&params.text_document.uri);
                let mut changes = params.content_changes.into_iter();
                let change = changes.next();
                if changes.next().is_none() {
                    if let Some(change) = change.filter(|change| change.range.is_none()) {
                        let _ = documents.change(
                            &params.text_document.uri,
                            params.text_document.version,
                            change.text,
                        );
                    }
                }
            }
        }
        "textDocument/didClose" => {
            if let Ok(params) =
                serde_json::from_value::<DidCloseTextDocumentParams>(notification.params)
            {
                documents.close(&params.text_document.uri);
            }
        }
        _ => {}
    }
}

#[derive(Clone, Copy)]
pub struct ServerError;

impl std::fmt::Debug for ServerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ServerError(INTERNAL_ERROR)")
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("INTERNAL_ERROR")
    }
}

impl std::error::Error for ServerError {}
