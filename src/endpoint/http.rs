use std::net::SocketAddr;

use ijr::{JsonResponseMiddleware};
use iron::prelude::*;

use endpoint::routes::load_routes;


pub fn start_server(addr: SocketAddr) {
    let router = load_routes();
    router.list_routes();

    let mut chain = Chain::new(router);
    chain.link_after(JsonResponseMiddleware);

    Iron::new(chain).http(addr).unwrap();
    info!("rustrix is now running at: {:?}", addr);
}
