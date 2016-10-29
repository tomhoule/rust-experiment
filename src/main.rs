extern crate jsonrpc_core;
extern crate nvim;

#[macro_use]
extern crate slog;

use slog::FilterLevel as LogLevel;

struct Config {
    log_level: LogLevel;
}

fn main() {
    println!("Hello, world!");
}
