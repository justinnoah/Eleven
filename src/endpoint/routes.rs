use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use slog;

use rustc_serialize::json;

// Routing setup //
pub struct Router {
    routes: HashMap<String, Box<Handler>>,
    log: slog::Logger,
}

impl Router {
    pub fn new(log: &slog::Logger) -> Self {
        let l = log.new(o!());
        Router {
            routes: HashMap::new(),
            log: l,
        }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }

    pub fn list_routes(&self) {
        let mut keys = Vec::new();
        for key in self.routes.keys() {
            keys.push(key);
        }
        slog_debug!(self.log, format!("{:?}", keys).to_string());
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        slog_info!(self.log, format!("req.url: {:?}", req.url).to_string());
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => {
                Ok(Response::with(status::NotFound))
            }
        }
    }
}

// Routes //
pub fn load_routes(server_log: &slog::Logger) -> Router {
    const API_VERSION: &'static str = "r0.2.0";
    const MATRIX_CLIENT: &'static str = "_matrix/client";

    let route_builder_log = server_log.new(o!());
    let mut router = Router::new(&server_log);

    router.add_route(format!("{}/versions", MATRIX_CLIENT), |_: &mut Request| {
        let versions = vec![API_VERSION.to_string()];
        let mut version_hash: HashMap<&str, Vec<String>> = HashMap::new();
        version_hash.insert("versions", versions);

        let mut resp = Response::new();
        resp.set_mut(json::encode(&version_hash).unwrap()).set_mut(status::Ok);
        Ok(resp)
    });

    // Ugly code, preping for the eventual support of multiple API versions
    use endpoint::client::load_client_routes;

    let client_routes: HashMap<String, Box<Handler>> = load_client_routes();
    for (r, f) in client_routes {
        let loc = format!("{}/{}", MATRIX_CLIENT, r);
        slog_info!(route_builder_log, loc.to_string());
        router.add_route(loc, f);
    }

    router
}
