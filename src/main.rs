#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate ini;
extern crate iron;
extern crate iron_json_response as ijr;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_envlogger;
extern crate slog_stdlog;
extern crate slog_term;

use docopt::Docopt;
use slog::DrainExt;

mod config;
mod endpoint;
use config::load_config;
use endpoint::http;


const USAGE: &'static str =  "
Usage: rustrix [-c <config>]

Options:
    -c, --config CONFIG  Path to configuration file
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_config: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let term = slog_term::streamer().build();
    let drain = slog_envlogger::new(term);
    let root_log = slog::Logger::root(drain.fuse(), o!("version" => "0.0.1"));
    slog_stdlog::set_logger(root_log.clone()).unwrap();

    slog_info!(root_log, "Loading Config");
    let cfg = load_config(args.flag_config.clone());

    http::start_server(cfg.http, root_log);
}
