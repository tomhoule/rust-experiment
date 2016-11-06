struct TextDocumentContentChangeEvent {
    range: Option<Range>,
    rangeLength: Option<i32>,
    text: String,
}

struct DidChangeTextDocumentParams {
    textDocument: VersionedTextDocumentIdentifier,
    contentChanges: Vec<TextDocumentContentChangeEvent>,
}

impl Notification for DidChangeTextDocumentParams {
    method = "textDocument/didChange";
}
