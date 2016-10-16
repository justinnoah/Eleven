extern crate docopt;
extern crate ini;
extern crate iron;
extern crate iron_json_response as ijr;
#[macro_use]
extern crate rustc_serialize;
#[macro_use]
extern crate slog;
extern crate slog_term;

mod config;
mod endpoint;

use docopt::Docopt;
use slog::DrainExt;

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

    let drain = slog_term::streamer().compact().build().fuse();

    let root_log = slog::Logger::root(drain, o!("version" => "0.0.1"));

    info!(root_log, "Loading Config");
    let cfg = load_config(args.flag_config.clone());

    http::start_server(cfg.address, root_log);
}
