extern crate docopt;
extern crate ini;
extern crate iron;
extern crate iron_json_response as ijr;
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rustc_serialize;

mod config;
mod endpont;

use docopt::Docopt;

use config::load_config;
use endpont::http;


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

    let cfg = load_config(args.flag_config.clone());
    http::start_server(cfg.address);
}
