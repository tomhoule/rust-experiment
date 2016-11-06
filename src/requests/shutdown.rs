struct ShutdownRequest;

impl Request for ShutdownRequest {
    fn make_message() -> RequestMessage<ShutdownRequest> {
        IncompleteRequestMessage {
            method: "shutdown",
            params: ShutdownRequest
        }
    }
}

struct ShutDownResponse;
