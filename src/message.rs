type ContentType = String;

impl Default for ContentType {
    fn default() -> ContentType {
        "application/vscode-jsonrpc; charset=utf8".to_string()
    }
}

struct Headers {
    content_length: Option<i32>;
    content_type: Option<String>;
}

impl Headers {
    fn produce(&self) -> String {
        "".to_string()
    }
}

struct Content;
struct Message;

fn assemble(header: Headers, content: Content) -> Message {
    // print!("{}\r\n{}");
    // header + '\r\n' + content
}


