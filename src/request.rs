struct RequestMessage<T> {
    id: i32;
    method: String;
    params: T;
}

impl RequestMessage<T> {
    pub fn new(method: String, params: T) -> RequestMessage<T> {
        RequestMessage {
            id: 0,
            method: method,
            params: params
        }
    }
}

pub trait Request {
    params = Self;

    fn make_message() -> RequestMessage<Self>;
}

