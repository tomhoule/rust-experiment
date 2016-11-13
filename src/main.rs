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

mod broker;
mod language_server;
mod message;
mod neovim;
mod requests;
mod supported_languages;
mod types;

use broker::*;
use slog::*;

/*
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

/// To iterate faster in development, use the NVIM_LISTEN_ADDRESS environment variable and set it
/// to 127.0.0.1:6666
fn main() {
    let logger = slog_term::streamer().stdout().full().build();
    let root = slog::Logger::root(logger.fuse(), None);

    info!(root, "Starting up");
    Broker::new(root).start();
}
