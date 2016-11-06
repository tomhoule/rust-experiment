struct LogMessageParams {
    type: i32, // messagetype
    message: String,
}

impl Notification for LogMessageParams {
    method = "window/logMessage";
}
