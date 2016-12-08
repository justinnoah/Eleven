use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use iron::prelude::*;
use jsonway;
use rusqlite::Connection;
use rustless::{Application, Api, Nesting};
use super::{DB, MatrixNamespace};


pub fn api_builder() -> Api {
    let mut api = Api::new();

    api.prefix("_matrix/client/");
    api.get("versions", |endpoint| {
        endpoint.handle(|client, _params| {
            let versions = jsonway::object(|json| {
                json.array("versions", |versions| {
                    versions.push("r0.2.0");
                });
            });
            let resp: String = versions.unwrap().to_string();
            debug!("Versions response: {}", resp);
            client.text(resp)
        })
    });
    api
}

pub fn start_server(cfg: Box<HashMap<String, String>>, namespace: &String, db_con: Connection) {
    let mut app = Application::new(api_builder());

    // Additional extensions to the app
    app.ext.insert::<MatrixNamespace>(namespace.clone());
    app.ext.insert::<DB>(db_con);

    let addr = match cfg.get("address") {
        Some(x) => x,
        None => "127.0.0.1:8448",
    };

    // TLS key check. Start with HTTP if no key path is found
    match cfg.get("key_path") {
        Some(x) => {
            info!("Starting server using HTTPS");
            let tls_path = Path::new(x);
            let crt_path = tls_path.join("domain.crt");
            let key_path = tls_path.join("domain.key");
            if crt_path.exists() && key_path.exists() {
                Iron::new(app).https(
                    SocketAddr::from_str(addr).unwrap(), crt_path, key_path).unwrap();
            } else {
                Iron::new(app).http(SocketAddr::from_str(addr).unwrap()).unwrap();
            }
        },
        None => {
            Iron::new(app).http(SocketAddr::from_str(addr).unwrap()).unwrap();
        }
    };
    info!("eleven is now running at: {:?}", addr);
}
