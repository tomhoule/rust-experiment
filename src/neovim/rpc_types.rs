use types::Position;
use rmp::value::{Value, Integer};

#[derive(Debug)]
pub enum RPCEvent {
    NewCursorPosition(Position),
    TextChangedI,
}

impl NeovimRPCEvent {
    pub fn new(event: &str, values: Vec<Value>) -> Option<RPCEvent> {
        match event {
            "language_server_new_cursor_position" => {
                if let (&Value::Integer(Integer::U64(line)),
                        &Value::Integer(Integer::U64(character)))
                        = (&values[0], &values[1]) {
                    let pos = Position {
                        line: line,
                        character: character,
                    };
                    Some(RPCEvent::NewCursorPosition(pos))
                } else {
                    panic!();
                }
            },
            "language_server_text_changed" => {
                Some(RPCEvent::TextChangedI)
            },
            _ => None
        }
    }
}
