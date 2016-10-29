use ::Request;

struct ClientCapabilities;

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
