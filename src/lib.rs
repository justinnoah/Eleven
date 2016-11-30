#![crate_name = "libeleven"]
extern crate hyper;
extern crate ini;
extern crate iron;
extern crate jsonway;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustless;
extern crate serde_json;
#[macro_use(o)]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_term;
extern crate typemap;
extern crate valico;

use slog::DrainExt;

pub mod actions;
pub mod db;
pub mod config;
pub mod errors;
pub mod rest;
use config::Config;
use rest::server;


fn rest_up(cfg: Config) {
    let root_log = slog::Logger::root(slog_term::streamer().stderr().build().fuse(), o!());
    slog_stdlog::set_logger(root_log.clone()).unwrap();

    info!("Loading REST Interface");
    server::start_server(cfg.http, cfg.matrix.get("namespace").unwrap());
}

pub fn entry(cfg: Config) {
    // Init the db
    let pool = db::init_db(cfg.matrix.get("").unwrap());

    // For now, Matrix only supports a REST protocol, this should start here
    rest_up(cfg);
}
