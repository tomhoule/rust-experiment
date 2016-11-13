#![feature(question_mark)]
#![feature(io)]
#![feature(proc_macro)]
#![feature(conservative_impl_trait)]
#![feature(unboxed_closures)]

#[macro_use] extern crate chomp;
extern crate futures;
extern crate jsonrpc_core;
extern crate libc;
extern crate neovim_lib;
extern crate regex;
extern crate rmp;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate uuid;

mod language_server;
mod message;
mod neovim;
mod requests;
mod supported_languages;
mod types;

use neovim::NeovimRPCEvent;
use neovim_lib::{Neovim, NeovimApi, Session};
use language_server::*;

use futures::stream::{Stream, BoxStream};
use futures::{Async, Future, Poll};
use slog::*;
use std::sync::mpsc;

type NeovimRPCError = ();
type LanguageServerRPCError = ();
type LanguageServerRPCEvent = ();

// struct Broker<S, T>
//     where S: Stream<Item=NeovimRPCEvent, Error=NeovimRPCError>,
//           T: Stream<Item=LanguageServerRPCEvent, Error=LanguageServerRPCError>
// {
//      neovim: Neovim,
//      neovim_events: S,
//      language_server: LanguageServerWrapper,
// }

struct NeovimManager {
    neovim: Neovim,
    events: NeovimEventsHandler,
    // events: BoxStream<NeovimRPCEvent, NeovimRPCError>,
}

impl NeovimManager {
    fn new(logger: Logger) -> Self {
        let nvim_session = Session::new_tcp("127.0.0.1:6666").unwrap();
        let mut nvim = Neovim::new(nvim_session);
        let event_handler = NeovimEventsHandler::new(logger, &mut nvim);

        NeovimManager {
            neovim: nvim,
            events: event_handler,
        }
    }
}

struct Broker {
    neovim: NeovimManager,
    language_server: LanguageServerManager,
    logger: Logger,
}

impl Broker {
    fn new(logger: Logger) -> Self {

        Broker {
            neovim: NeovimManager::new(logger),
            language_server: LanguageServerManager::new(logger),
            logger: logger,
        }
    }

    fn start(&self) {
        self.neovim.events.for_each(|event| {
            self.neovim.neovim.command(&format!("echo \"{:?}\"", event));
            debug!(self.logger, "{:?}", event);
            // manager.handle_event(event);
            Ok(())
        }).wait();
    }
}

// impl Broker<S, T> {
//     fn change_file(path: &str) -> Result<(), String> { unimplemented!() }
//     fn display_completion(candidates: Vec<String>) -> Result<(), String> { unimplemented!() }
//     fn go_to_definition(symbol: &str) -> Result<(), String> { unimplemented!() }
//     fn poll() { unimplemented!() }
// }

/*
 *
 * I should buy a boat.
 *
 * And put everything, including receivers/emitters and the threadpool, in one struct
 *
 * Don't forget checking that the marks are preserved when changing buffer text
 *
 */

// au -> InsertEnter
// au -> InsertLeave
// au -> TextChangedI
// au -> CompleteDone
// au -> BufNew, BufNewFile, BufReadPost
// au -> CursorMove

struct NeovimEventsHandler {
    logger: Logger,
    // neovim: Neovim,
    receiver: mpsc::Receiver<NeovimRPCEvent>
}

impl NeovimEventsHandler {
    pub fn new(logger: Logger, nvim: &mut Neovim) -> Self {
        let (sender, receiver) = mpsc::channel::<NeovimRPCEvent>();

        info!(logger, "Starting the neovim event loop");
        nvim.session.start_event_loop_cb(move |event, values| {
            sender.send(NeovimRPCEvent::new(event, values).unwrap());
        });

        info!(logger, "Subscribing to neovim events");
        for event_type in NEOVIM_EVENT_TYPES {
            nvim.subscribe(event_type);
        }

        NeovimEventsHandler {
            logger: logger,
            receiver: receiver,
        }
    }
}

impl Stream for NeovimEventsHandler {
    type Item = NeovimRPCEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<NeovimRPCEvent>, ()> {
        match self.receiver.recv() {
            Ok(event) => Ok(Async::Ready(Some(event))),
            Err(_) => Ok(Async::NotReady),
        }
    }
}

const NEOVIM_EVENT_TYPES: &'static [&'static str] = &[
    "language_server_new_cursor_position",
    "language_server_text_changed",
    "lsp/bufread"
];

/// To iterate faster in development, use the NVIM_LISTEN_ADDRESS environment variable and set it
/// to 127.0.0.1:6666
fn main() {
    let logger = slog_term::streamer().stdout().full().build();
    let root = slog::Logger::root(logger.fuse(), None);

    info!(root, "Starting up");

}
