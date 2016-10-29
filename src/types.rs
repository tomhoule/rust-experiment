pub struct Position {
    line: i32;
    character: i32;
}

/**
 * The end position is exclusive.
 */
pub struct Range {
    start: Position;
    end: Position;
}

pub struct Location {
    uri: String;
    range: Range;
}

pub struct Diagnostic {
    range: Range;
    severity: Option<DiagnosticSeverity>;
    code: Option<i32>;
    source: Option<String>;
    message: String;
}

enum DiagnosticSeverity {
    Error, // 1
    Warning, // 2
    Information, // 3
    Hint, // 4
}

/**
 * TODO: clarify this
 */
pub struct Command<T> {
    title: String;
    command: String;
    arguments?: Vec<T>;
}

pub struct TextEdit {
    range: Range;
    newText: String;
}

type URI = String;

pub struct WorkSpaceEdit {
    changes: Map<URI, TextEdit>;
}

pub struct TextDocumentIdentifier {
    uri: URI;
}

pub struct TextDocumentItem {
    uri: URI;
    languageId: String;
    version: i32;
    text: String;
}

pub struct VersionedTextDocumentIdentifier {
    uri: URI;
    languageId: String;
    version: i32;
    text: String;
    version: i32;
}

pub struct TextDocumentPositionParams {
    textDocument: TextDocumentIdentifier;
    position: Position;
}
