extern crate docopt;
extern crate libeleven;
extern crate rustc_serialize;

use docopt::Docopt;
use libeleven::config::load_config;
use libeleven::rest_up;


const USAGE: &'static str =  "
Usage: eleven [-c <config>]

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
    rest_up(cfg);
}
