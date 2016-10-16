use std::net::{SocketAddr};

use ijr::{JsonResponseMiddleware};
use iron::prelude::*;
use slog;

use endpoint::routes::load_routes;

pub fn start_server(addr: SocketAddr, root_log: slog::Logger) {
    let server_log = root_log.new(o!("address" => addr.to_string()));

    info!(server_log, "Loading Routes");
    let router = load_routes(&server_log);
    router.list_routes();

    let mut chain = Chain::new(router);
    chain.link_after(JsonResponseMiddleware);

    Iron::new(chain).http(addr).unwrap();
    info!(server_log, "rustrix is now running at: {:?}", addr);
}
