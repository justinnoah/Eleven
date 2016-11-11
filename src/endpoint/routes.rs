use std::collections::HashMap;
use std::io::Read;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use serde_json as json;


type Routes = HashMap<String, Box<Handler>>;

// Routing setup //
pub struct Router {
    routes: Routes,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
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
        debug!("{:?}", keys);
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        info!("req.url: {:?}", req.url);
        debug!("req.method: {:?}", req.method);
        let mut body = String::new();
        let _ = req.body.read_to_string(&mut body);
        // debug!("Data Dump: {:?}", body);
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => {
                Ok(Response::with(status::NotFound))
            }
        }
    }
}

// Routes //
pub fn load_routes() -> Router {
    const API_VERSION: &'static str = "r0.2.0";
    const MATRIX_CLIENT: &'static str = "_matrix/client";

    let mut router = Router::new();

    router.add_route(format!("{}/versions", MATRIX_CLIENT), |_: &mut Request| {
        let versions = vec![API_VERSION.to_string()];
        let mut version_hash: HashMap<&str, Vec<String>> = HashMap::new();
        version_hash.insert("versions", versions);

        let mut resp = Response::new();
        resp.set_mut(json::to_string(&version_hash).unwrap()).set_mut(status::Ok);
        Ok(resp)
    });

    // Ugly code, preping for the eventual support of multiple API versions
    use endpoint::client::load_client_routes;

    let client_routes: HashMap<String, Box<Handler>> = load_client_routes();
    for (r, f) in client_routes {
        let loc = format!("{}/{}", MATRIX_CLIENT, r);
        info!("{}", loc);
        router.add_route(loc, f);
    }

    router
}
