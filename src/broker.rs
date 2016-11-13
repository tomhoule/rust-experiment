use futures::{Async, Future, Poll};
use futures::stream::{Stream };
use language_server::*;
use neovim::*;
use slog::*;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::thread;

#[derive(Debug)]
pub enum Event {
    Neovim(NeovimRPCEvent),
    LanguageServer(LsRpcEvent),
}

pub struct Broker {
    neovim: Neovim,
    neovim_events: NeovimEventStream,
    language_server: LanguageServerManager, // should be in a thread, + channel, and return (?)
    logger: Logger,
    // threadpool
}

impl Broker {
    pub fn new(logger: Logger) -> Self {
        let (neovim, neovim_events) = attach_to_neovim(logger.clone());

        Broker {
            neovim: neovim,
            neovim_events: neovim_events,
            language_server: LanguageServerManager::new(logger.clone()),
            logger: logger,
        }
    }

    pub fn start(&self) {
        thread::spawn(|| {
            // vecdeque pop
            // sleep
        });
    }
}

// impl Broker<S, T> {
//     fn change_file(path: &str) -> Result<(), String> { unimplemented!() }
//     fn display_completion(candidates: Vec<String>) -> Result<(), String> { unimplemented!() }
//     fn go_to_definition(symbol: &str) -> Result<(), String> { unimplemented!() }
//     fn poll() { unimplemented!() }
// }

