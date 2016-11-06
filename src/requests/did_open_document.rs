struct DidOpenTextDocumentParams {
    textDocument: TextDocumentItem,
}

impl Notification for DidOpenTextDocumentParams {
    method = "textDocument/didOpen";
}
