enum MessageType {
    Error, // 1
    Warning, // 2
    Info, // 3
    Log, // 4
}

struct ShowMessageNotificationParams {
    type: MessageType,
    message: String,
}

impl Notification for ShowMessageNotificationParams {
    method = "window/showMessage"
}

struct MessageActionItem {
    title: String,
}

struct ShowMessageRequestParams {
    type: i32,
    message: String,
    actions: Option<Vec<MessageActionItem>>
}

impl ServerRequest for ShowMessageRequestParams {
    method = "window/showMessageRequest"
}

// Response: a MessageActionItem
