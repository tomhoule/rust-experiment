enum ErrorCode {
    ParseError, // -32700
    InvalidRequest, // -32600
    MethodNotFound, // -32601
    InvalidParams, // -32602
    InternalError, // -32603
    serverErrorStart, // -32099
    serverErrorEnd, // -32000
}

struct ResponseError<D> {
    code: ErrorCode;
    message: String;
    data: Option<D>;
}

struct ResponseMessage<T, E> {
    id: i32;
    result: Option<T>;
    error?: Option<ResponseError<E>>;
}
