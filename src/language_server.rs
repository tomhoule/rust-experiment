use slog::Logger;
use std::process::{Child, Command};
use std::io::{Read, Write};
use std::marker::PhantomData;
use supported_languages::SupportedLanguage;
use neovim::NeovimRPCEvent;
use requests::*;
use futures::{Future};


pub struct LanguageServerManager {
    logger: Logger,
    server: Option<LanguageServerWrapper>,
}

impl LanguageServerManager {
    pub fn new(logger: Logger) -> Self {
        LanguageServerManager {
            logger: logger,
            server: None,
        }
    }

    pub fn handle_event(&mut self, event: NeovimRPCEvent) {
        match event {
            NeovimRPCEvent::BufRead(lang) => {
                self.server = Some(LanguageServerWrapper::new(lang, self.logger.clone()));
            }
            _ => {
                if let Some(ref mut server) = self.server {
                    server.handle_event(event);
                }
            }
        }
    }
}

pub enum LanguageServerWrapper {
    Starting(LanguageServer<Starting>),
    Running(LanguageServer<Running>),
    Failed(LanguageServer<Failed>),
}

impl LanguageServerWrapper {
    fn new(lang: SupportedLanguage, logger: Logger) -> Self {
        let new_server = LanguageServer::<Starting>::new(lang, logger.clone());
        let mut started_server = new_server.start();
        // let handler = RequestHandler::new().add_method("initialize", InitializeRequest);
        LanguageServerWrapper::Running(started_server)
    }

    fn handle_event(&mut self, event: NeovimRPCEvent) {
        match self {
            &mut LanguageServerWrapper::Running(ref mut server) => server.handle_event(event),
            _ => (),
        }
    }
}


struct Starting { }

/* Maybe a threadpool with futures
 * The stream is completely synchronous (fd io buffering) but we can handle the requests
 * asynchronously. ServerRequest -> Future<ClientResponse>
 */
struct Running {
    // text_buffer: String,
    // stdout_buffer: String,
    process: Child,
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
    language: SupportedLanguage,
    logger: Logger,
    state: S,
}

impl LanguageServer<Starting> {
    fn new(lang: SupportedLanguage, logger: Logger) -> Self {
        LanguageServer {
            logger: logger,
            language: lang,
            state: Starting { }
        }
    }

    fn start(self) -> LanguageServer<Running> {
        info!(self.logger, &format!("Starting {:?} language server", self.language));
        let child = self.language.start_language_server().unwrap();
        let mut child = Command::new("./node_modules/typescript/bin/tsserver").spawn().unwrap();

        LanguageServer {
            logger: self.logger,
            language: self.language,
            state: Running {
                process: child
                // child_stdin: child_stdin,
            }
        }
    }
}

impl LanguageServer<Running> {
    pub fn send(&mut self, request: &str) {
        let message = format!("{}", request);
        debug!(self.logger, "Sending {:?}", message);
        self.state.process.stdin.as_mut().map(|s| {
            s.write("Content-Length: ".as_bytes());
            s.write(format!("{}", message.len()).as_bytes());
            s.write("\r\n\r\n".as_bytes());
            s.write(message.as_bytes());
        });
    }

    fn handle_event(&mut self, event: NeovimRPCEvent) {
        // if it has a request id, resolve the corresponding future
        // otherwise, forward it to the main stream
        let mut buf = String::new();
        let mut state = &mut self.state;
        let mut process = &mut state.process;
        process.stdout.as_mut().map(|mut s| s.read_to_string(&mut buf));
        println!("Received {}", buf);
    }

    fn notify(notification: String) { }
    fn request(request: String) -> Box<Future<Item=i32, Error=i32>> { unimplemented!() }
}
