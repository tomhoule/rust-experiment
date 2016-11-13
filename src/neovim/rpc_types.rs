use types::Position;
use rmp::value::{Value, Integer};
use supported_languages::SupportedLanguage;
use std::str::FromStr;

#[derive(Debug)]
pub enum NeovimRPCEvent {
    NewCursorPosition(Position),
    TextChangedI,
    BufRead(SupportedLanguage),
}

impl NeovimRPCEvent {
    pub fn new(event: &str, values: Vec<Value>) -> Option<Self> {
        match event {
            "language_server_new_cursor_position" => {
                if let (&Value::Integer(Integer::U64(line)),
                        &Value::Integer(Integer::U64(character)))
                        = (&values[0], &values[1]) {
                    let pos = Position {
                        line: line,
                        character: character,
                    };
                    Some(NeovimRPCEvent::NewCursorPosition(pos))
                } else {
                    None
                }
            },
            "language_server_text_changed" => {
                Some(NeovimRPCEvent::TextChangedI)
            },
            "lsp/bufread" => {
                if let Value::String(ref lang) = values[0] {
                    Some(NeovimRPCEvent::BufRead(SupportedLanguage::from_str(lang).unwrap()))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}
