//! Versioned document and preview state.

use std::collections::HashMap;
use std::fmt;

use lsp_types::{Range, Uri};
use translator_core::{
    ErrorCode, InputKind, ProviderConfiguration, ProviderMode, ProviderSelection, TranslateFailure,
};

#[derive(Clone)]
pub struct DocumentSnapshot {
    uri: Uri,
    version: i32,
    input_kind: InputKind,
    text: String,
}

impl DocumentSnapshot {
    pub const fn version(&self) -> i32 {
        self.version
    }

    pub const fn input_kind(&self) -> InputKind {
        self.input_kind
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

impl fmt::Debug for DocumentSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentSnapshot")
            .field("version", &self.version)
            .field("input_kind", &self.input_kind)
            .field("text_bytes", &self.text.len())
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct TranslationPreview {
    uri: Uri,
    version: i32,
    source_range: Range,
    translated_text: String,
    input_kind: InputKind,
}

impl TranslationPreview {
    pub fn new(
        uri: Uri,
        version: i32,
        source_range: Range,
        translated_text: String,
        input_kind: InputKind,
    ) -> Self {
        Self {
            uri,
            version,
            source_range,
            translated_text,
            input_kind,
        }
    }

    pub const fn version(&self) -> i32 {
        self.version
    }

    pub const fn source_range(&self) -> Range {
        self.source_range
    }

    pub fn translated_text(&self) -> &str {
        &self.translated_text
    }

    pub const fn input_kind(&self) -> InputKind {
        self.input_kind
    }
}

impl fmt::Debug for TranslationPreview {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TranslationPreview")
            .field("version", &self.version)
            .field("source_range", &self.source_range)
            .field("input_kind", &self.input_kind)
            .field("translated_bytes", &self.translated_text.len())
            .finish_non_exhaustive()
    }
}

#[derive(Default)]
pub struct DocumentStore {
    documents: HashMap<Uri, DocumentSnapshot>,
    previews: HashMap<Uri, TranslationPreview>,
}

impl DocumentStore {
    pub fn open(
        &mut self,
        uri: Uri,
        version: i32,
        language_id: &str,
        text: String,
    ) -> Result<(), TranslateFailure> {
        let input_kind = input_kind_from_language(language_id)?;
        self.previews.remove(&uri);
        self.documents.insert(
            uri.clone(),
            DocumentSnapshot {
                uri,
                version,
                input_kind,
                text,
            },
        );
        Ok(())
    }

    pub fn change(
        &mut self,
        uri: &Uri,
        version: i32,
        text: String,
    ) -> Result<(), TranslateFailure> {
        self.previews.remove(uri);
        let snapshot = self.documents.get_mut(uri).ok_or_else(invalid_input)?;
        if version <= snapshot.version {
            return Err(invalid_input());
        }
        snapshot.version = version;
        snapshot.text = text;
        Ok(())
    }

    pub fn close(&mut self, uri: &Uri) {
        self.documents.remove(uri);
        self.previews.remove(uri);
    }

    pub fn get(&self, uri: &Uri) -> Option<&DocumentSnapshot> {
        self.documents.get(uri)
    }

    pub fn set_preview(&mut self, preview: TranslationPreview) {
        self.previews.insert(preview.uri.clone(), preview);
    }

    pub fn preview(&self, uri: &Uri) -> Option<&TranslationPreview> {
        self.previews.get(uri)
    }

    pub fn invalidate_preview(&mut self, uri: &Uri) {
        self.previews.remove(uri);
    }
}

impl fmt::Debug for DocumentStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentStore")
            .field("document_count", &self.documents.len())
            .field("preview_count", &self.previews.len())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderLocalityLabel {
    Offline,
    Local,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderDescriptor {
    locality: ProviderLocalityLabel,
    allow_remote: bool,
}

/// Provider selection and safe descriptor produced from one configuration.
pub struct ProviderRuntime {
    selection: ProviderSelection,
    descriptor: ProviderDescriptor,
}

impl ProviderRuntime {
    /// Consume one validated configuration to build execution and locality state.
    ///
    /// # Errors
    ///
    /// Returns the shared provider configuration error when selection cannot
    /// be constructed, including a missing referenced remote credential.
    pub fn from_configuration(
        configuration: ProviderConfiguration,
    ) -> Result<Self, TranslateFailure> {
        let descriptor = ProviderDescriptor::from_configuration(&configuration);
        let selection = ProviderSelection::from_configuration(configuration)?;
        Ok(Self {
            selection,
            descriptor,
        })
    }

    /// Split the already-matched execution provider and safe UI descriptor.
    pub fn into_parts(self) -> (ProviderSelection, ProviderDescriptor) {
        (self.selection, self.descriptor)
    }
}

impl fmt::Debug for ProviderRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderRuntime")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl ProviderDescriptor {
    pub const fn offline() -> Self {
        Self {
            locality: ProviderLocalityLabel::Offline,
            allow_remote: false,
        }
    }

    pub const fn local() -> Self {
        Self {
            locality: ProviderLocalityLabel::Local,
            allow_remote: false,
        }
    }

    pub const fn remote(allow_remote: bool) -> Self {
        Self {
            locality: ProviderLocalityLabel::Remote,
            allow_remote,
        }
    }

    pub fn from_configuration(configuration: &ProviderConfiguration) -> Self {
        match configuration.mode {
            ProviderMode::Mock => Self::offline(),
            ProviderMode::EmbeddedLocal => Self::offline(),
            ProviderMode::LibreTranslate => Self::local(),
            ProviderMode::AzureTranslator => Self::remote(true),
        }
    }

    pub const fn locality(self) -> ProviderLocalityLabel {
        self.locality
    }

    pub const fn allow_remote(self) -> bool {
        self.allow_remote
    }

    pub const fn action_title(self) -> &'static str {
        match self.locality {
            ProviderLocalityLabel::Offline => "Translate English to Spanish [offline]",
            ProviderLocalityLabel::Local => "Translate English to Spanish [local]",
            ProviderLocalityLabel::Remote => {
                "Translate English to Spanish [remote - confirmation required]"
            }
        }
    }
}

fn input_kind_from_language(language_id: &str) -> Result<InputKind, TranslateFailure> {
    match language_id.to_ascii_lowercase().as_str() {
        "markdown" => Ok(InputKind::Markdown),
        "plaintext" | "plain text" => Ok(InputKind::Text),
        _ => Err(invalid_input()),
    }
}

fn invalid_input() -> TranslateFailure {
    TranslateFailure::new(ErrorCode::InvalidInput, "Invalid document state.")
}
