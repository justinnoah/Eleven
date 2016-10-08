use std::net::SocketAddr;
use std::thread;

use ijr::{JsonResponseMiddleware};
use iron::prelude::*;

use endpont::routes::load_routes;


pub fn start_server(addr: SocketAddr) {
    let router = load_routes();
    router.list_routes();

    let mut chain = Chain::new(router);
    chain.link_after(JsonResponseMiddleware);

    println!("Begin running at: {:?}", addr);
    Iron::new(chain).http(addr).unwrap();
}
