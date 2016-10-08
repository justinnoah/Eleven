use std::collections::HashMap;

use ijr::{JsonResponse};
use iron::prelude::*;
use iron::{Handler};
use iron::status;
use rustc_serialize::json;



// Routing setup //

pub struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }

    pub fn list_routes(&self) {
        let mut keys = Vec::new();
        for key in self.routes.keys() {
            keys.push(key);
        }
        println!("{:?}", keys);
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => {
                println!("req.url: {:?}", req.url);
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
        resp.set_mut(json::encode(&version_hash).unwrap()).set_mut(status::Ok);
        Ok(resp)
    });

    router
}

