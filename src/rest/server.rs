use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use iron::prelude::*;
use jsonway;
use rustless::{Application, Api, Nesting};

pub fn api_builder() -> Api {
    let mut api = Api::new();

    api.prefix("_matrix/client/");
    api.get("versions", |endpoint| {
        endpoint.handle(|client, _params| {
            let versions = jsonway::array(|json| {
                json.push("r0.2.0");
            });
            let resp: String = versions.unwrap().to_string();
            debug!("Versions response: {}", resp);
            client.text(resp)
        })
    });
    api
}


pub fn start_server(cfg: Box<HashMap<String, String>>) {
    // Config options needed for this  function
    let tls_path = Path::new(cfg.get("key_path").unwrap());
    let crt_path = tls_path.join("domain.crt");
    let key_path = tls_path.join("domain.key");
    let addr = cfg.get("address").unwrap();

    let app = Application::new(api_builder());

    if crt_path.exists() && key_path.exists() {
        Iron::new(app).https(
            SocketAddr::from_str(addr).unwrap(), crt_path, key_path).unwrap();
    } else {
        Iron::new(app).http(SocketAddr::from_str(addr).unwrap()).unwrap();
    }
    info!("eleven is now running at: {:?}", addr);
}
