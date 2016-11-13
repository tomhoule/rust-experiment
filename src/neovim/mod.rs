use futures::stream::{Stream, BoxStream, MergedItem};
use futures::{Async, Future, Poll};
pub use neovim_lib::{Neovim, NeovimApi, Session};
use slog::*;
use std::sync::mpsc;
use std::sync::Mutex;
use std::collections::VecDeque;
use broker::Event;

mod rpc_types;

pub use self::rpc_types::NeovimRPCEvent;

pub type NeovimRPCError = ();

pub fn attach_to_neovim(logger: Logger) -> (Neovim, NeovimEventStream) {
    let nvim_session = Session::new_tcp("127.0.0.1:6666").unwrap();
    let mut nvim = Neovim::new(nvim_session);
    let event_handler = NeovimEventStream::new(logger, &mut nvim);
    (nvim, event_handler)
}

const NEOVIM_EVENT_TYPES: &'static [&'static str] = &[
    "language_server_new_cursor_position",
    "language_server_text_changed",
    "lsp/bufread"
];

pub struct NeovimEventStream {
    logger: Logger,
    receiver: mpsc::Receiver<NeovimRPCEvent>
}

impl NeovimEventStream {
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

        NeovimEventStream {
            logger: logger,
            receiver: receiver,
        }
    }
}

impl Stream for NeovimEventStream {
    type Item = NeovimRPCEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<NeovimRPCEvent>, ()> {
        match self.receiver.recv() {
            Ok(event) => Ok(Async::Ready(Some(event))),
            Err(_) => Ok(Async::NotReady),
        }
    }
}
