#![crate_name = "libeleven"]
extern crate hyper;
extern crate ini;
extern crate iron;
extern crate jsonway;
#[macro_use]
extern crate log;
extern crate rustless;
extern crate serde_json;
#[macro_use(o)]
extern crate slog;
extern crate slog_stdlog;
extern crate valico;

pub mod actions;
pub mod config;
pub mod rest;
use config::Config;
use rest::server;


pub fn rest_up(cfg: Config) {
    let root_log = slog::Logger::root(slog::Discard, o!("version" => "0.1.0"));
    slog_stdlog::set_logger(root_log.clone()).unwrap();

    info!("Loading REST Interface");
    server::start_server(cfg.http);
}
