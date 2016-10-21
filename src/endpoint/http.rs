use std::collections::{HashMap};
use std::net::{SocketAddr};
use std::path::{Path};
use std::str::FromStr;

use ijr::{JsonResponseMiddleware};
use iron::prelude::*;
use slog;

use endpoint::routes::load_routes;


pub fn start_server(cfg: Box<HashMap<String, String>>, root_log: slog::Logger) {
    // Config options needed for this  function
    let tls_path = Path::new(cfg.get("key_path").unwrap());
    let crt_path = tls_path.join("domain.crt");
    let key_path = tls_path.join("domain.key");
    let addr = cfg.get("address").unwrap();

    // Create path from config if it doesn't exist
    if !tls_path.exists() {
        panic!("TLS Key Path not found");
    }

    // Server Log
    let server_log = root_log.new(o!("address" => addr.to_string()));

    // Load the routes needed by the server
    slog_info!(server_log, "Loading All Routes");
    let router = load_routes(&server_log);
    router.list_routes();

    // Chain our routes and MiddleWarez
    let mut chain = Chain::new(router);
    chain.link_after(JsonResponseMiddleware);

    if crt_path.exists() && key_path.exists() {
        Iron::new(chain).https(
            SocketAddr::from_str(addr).unwrap(), crt_path, key_path).unwrap();

        slog_info!(server_log, "rustrix is now running at: {:?}", addr);
    } else {
        panic!("Missing either 'domain.crt' or 'domain.key'");
    }

}
