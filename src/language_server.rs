use slog::Logger;
use std::process::{Child, Command};
use std::io::{Read, Write};
use std::marker::PhantomData;
use supported_languages::SupportedLanguage;
use neovim::NeovimRPCEvent;
use requests::*;
use futures::{Async, Future, Poll};
use futures::stream::{Stream, BoxStream, Filter, empty};
use std::sync::mpsc;
use std::sync::Mutex;
use uuid::Uuid;
use std::thread::spawn;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use std::collections::VecDeque;
use broker::Event;

type LsEventStream = BoxStream<LsRpcEvent, ()>;

#[derive(Debug, Deserialize)]
struct Response {
    id: Uuid,
}

#[derive(Debug, Deserialize)]
struct Notification { }

struct ResponseObserver {
    request_id: Uuid,
    response: Option<Response>,
}

/// Filters a stream of events, and maybe resolves a response future
struct ResponseListener {
    observers: Mutex<Vec<ResponseObserver>>,
}

impl ResponseListener {
    pub fn new() -> Self {
        ResponseListener {
            observers: Mutex::new(Vec::new()),
        }
    }
}

impl ResponseListener {
    fn filter_events(&mut self, event: LsRpcEvent) -> bool {
        if let LsRpcEvent::Response(response) = event {
            let mut observers = self.observers.lock().unwrap();
            let response = response;
            for observer in observers.iter_mut() {
                if observer.request_id == response.id {
                    observer.response = Some(response);
                    break
                }
            }
            false
        } else {
            true
        }
    }
}

pub struct LanguageServerManager {
    logger: Logger,
    server: Option<LanguageServerWrapper>,
    pub events: BoxStream<LsRpcEvent, ()>,
}

impl LanguageServerManager {
    pub fn new(logger: Logger) -> Self {
        LanguageServerManager {
            logger: logger,
            server: None,
            events: empty().boxed(),
        }
    }

    pub fn bufread(&mut self, language: SupportedLanguage) {
        self.server = Some(LanguageServerWrapper::new(language, self.logger.clone()));
    }

    pub fn request() { }
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
        LanguageServerWrapper::Running(started_server)
    }
}


struct Starting { }

#[derive(Debug)]
pub enum LsRpcEvent {
    Response(Response),
    Notification(Notification),
}

impl Deserialize for LsRpcEvent {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        if let Ok(response) = Response::deserialize(deserializer) {
            Ok(LsRpcEvent::Response(response))
        } else {
            Notification::deserialize(deserializer).and_then(|notification| {
                Ok(LsRpcEvent::Notification(notification))
            })
        }
    }
}

#[derive(Serialize)]
struct LsRpcRequest {
    jsonrpc: &'static str,
    id: Uuid,
    method: String,
    params: String,
}

impl LsRpcRequest {
    pub fn new(method_name: String, params: String) -> Self {
        LsRpcRequest {
            jsonrpc: "2.0",
            id: Uuid::new_v4(),
            method: method_name,
            params: params,
        }
    }

    pub fn write(&self, w: &mut Write) {
        let request_string = serde_json::to_string(self).unwrap();
        w.write("Content-Length: ".as_bytes());
        w.write(format!("{}", request_string.len()).as_bytes());
        w.write("\r\n\r\n".as_bytes());
        w.write(request_string.as_bytes());
    }
}

/* Maybe a threadpool with futures
 * The stream is completely synchronous (fd io buffering) but we can handle the requests
 * asynchronously. ServerRequest -> Future<ClientResponse>
 */
struct Running {
    // requests: HashMap<UUID, Box<Future<Item=i32, Error=i32>>>,
    // FIFO queue with the responses
    // id -> Future to be resolved with the response
    requests_in: mpsc::Sender<LsRpcRequest>,
    events_out: mpsc::Receiver<LsRpcEvent>,
    response_listener: ResponseListener,
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

    fn spawn_handler(&self, process: Child) -> Running {
        let (event_sender, event_receiver) = mpsc::channel::<LsRpcEvent>();
        let (request_sender, request_receiver) = mpsc::channel::<LsRpcRequest>();
        spawn(move || {
            let mut process = process;
            loop {
                let mut buf = String::new();
                process.stdout.as_mut().map(|mut s| s.read_to_string(&mut buf));
                event_sender.send(serde_json::from_str::<LsRpcEvent>(&buf).unwrap());

                if let Ok(request) = request_receiver.recv() {
                    process.stdin.as_mut().map(|mut s| request.write(s));
                }

                // sleep
            }
        });

        Running {
            events_out: event_receiver,
            requests_in: request_sender,
            response_listener: ResponseListener::new(),
        }
    }

    fn start(self) -> LanguageServer<Running> {
        info!(self.logger, &format!("Starting {:?} language server", self.language));
        let child = self.language.start_language_server().unwrap();
        let running = self.spawn_handler(child);

        LanguageServer {
            logger: self.logger,
            language: self.language,
            state: running,
        }
    }
}

impl LanguageServer<Running> {
    pub fn request(&mut self, request: String) -> Box<Future<Item=Response, Error=()>> {
        // spawns a thread, sends the request, listens on the stream for the response
        // sends the requests, spawn a thread and listen
        let uuid = "1234";
        unimplemented!()
    }
}

impl Stream for Running {
    type Item = LsRpcEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, ()> {
        match self.events_out.recv() {
            Ok(event) => Ok(Async::Ready(Some(event))),
            Err(_) => Ok(Async::NotReady),
        }
    }
}
