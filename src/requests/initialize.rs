use ::Request;

struct ClientCapabilities;

enum TextDocumentSyncKind {
    None, // 0
    Full, // 1
    Incremental, // 2
}

struct CompletionOptions {
    resolveProvider: Option<bool>,
    triggerCharacters: Option<Vec<String>>,
}

struct SignatureHelpOptions {
    triggerCharacters: Option<Vec<String>>,
}

struct CodeLensOptions {
    resolveProvider: Option<bool>,
}

struct DocumentOnTypeFormattingOptions {
    firstTriggerCharacter: String,
    moreTriggerCharacter: Option<Vec<String>>,
}

struct ServerCapabilities {
    textDocumentSync: Option<TextDocumentSyncKind>,
    hoverProvider: Option<bool>,
    completionProvider: Option<CompletionOptions>,
    signatureHelpProvider: Option<SignatureHelpOptions>,
    definitionProvider: Option<bool>,
    referencesProvider: Option<bool>,
    documentHighlightProvider: Option<bool>,
    documentSymbolProvider: Option<bool>,
    workspaceSymbolProvider: Option<bool>,
    codeActionProvider: Option<bool>,
    codeLensProvider: Option<CodeLensOptions>,
    documentFormattingProvider: Option<bool>,
    documentRangeFormattingProvider: Option<bool>,
    documentOnTypeFormattingProvider: Option<DocumentOnTypeFormattingOptions>,
    renameProvider: Option<bool>,
}

struct InitializeRequest {
    processId: Option<i32>;
    rootPath: Option<String>;
    initializationOptions: Option<String>;
    capabilities: ClientCapabilities;
}

impl Request for InitializeRequest {
    fn make_message() -> RequestMessage<Self> {
        IncompleteRequestMessage {
            method: "initialize",
            params: Self,
        }
    }
}

struct InitializeResult {
    capabilities: ServerCapabilities,
}

struct InitializeError {
    retry: bool,
}
