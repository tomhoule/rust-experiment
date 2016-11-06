use slog::Logger;
use std::process::*;
use std::marker::PhantomData;
use neovim::RPCEvent;

struct Request;
struct Response;

pub enum LanguageServerWrapper {
    Starting(LanguageServer<Starting>),
    Running(LanguageServer<Running>),
    Failed(LanguageServer<Failed>),
}

struct Starting { }

/* Maybe a threadpool with futures
 * The stream is completely synchronous (fd io buffering) but we can handle the requests
 * asynchronously. ServerRequest -> Future<ClientResponse>
 */
struct Running {
    text_buffer: String,
    stdout_buffer: String,
}

impl Running {
    fn poll(&self) -> Option<String> {
        // self.Child.stdout.read_to_string(self.stdout_buffer);
        // here we need to split the string into requests
        unimplemented!();
    }

    fn parse_message() {
        // read the header line by line until \r\n\r\n
        // read the size specified by the content-length header and serialize it
        // Message::read(reader) maybe?
    }
}

struct Failed {
    error: String,
}

pub struct LanguageServer<S> {
    process: Child,
    language: Language,
    logger: Logger,
    state: S,
}

pub trait LanguageServerBuilder {
    fn start(self) -> LanguageServer<Starting>;
}

pub enum Language {
    Typescript,
}

pub struct TSServer {
    logger: Logger,
}

impl TSServer {
    pub fn new(logger: Logger) -> Self {
        TSServer {
            logger: logger
        }
    }
}

impl LanguageServerBuilder for TSServer {
    fn start(self) -> LanguageServer<Starting> {
        info!(self.logger, "Starting tsserver");
        let child = Command::new("./node_modules/typescript/bin/tsserver").spawn().unwrap();
        LanguageServer {
            process: child,
            language: Language::Typescript,
            logger: self.logger,
            state: Starting { }
        }
    }
}
