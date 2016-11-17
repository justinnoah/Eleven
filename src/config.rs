/// cfg.ini Layout
///
/// ------------------------
/// [http]
/// address = "0.0.0.0:8448"
/// key_path = "tls/"
///
/// [matrix]
/// namespace = com.example
/// ------------------------
///
/// namespace must be a domain you own, preferably with TLS keys

use std::collections::{HashMap};

use ini::Ini;

pub struct Config {
    pub http: Box<HashMap<String, String>>,
    pub matrix: Box<HashMap<String, String>>,
}

impl Config {
    pub fn load_from_ini(cfg: Ini) -> Config {
        let http = match cfg.section(Some("http")) {
            Some(x) => x.clone(),
            None => {
                // Default http section
                let mut http_ = HashMap::<String, String>::new();
                http_.insert("address".to_string(), "127.0.0.1:8448".to_string());
                http_.insert("key_path".to_string(), "".to_string());
                http_
            }
        };

        let matrix = match cfg.section(Some("matrix")) {
            Some(x) => x.clone(),
            None => {
                // Default http section
                let mut matrix_ = HashMap::<String, String>::new();
                matrix_.insert("namespace".to_string(), "com.example".to_string());
                matrix_
            }
        };
        Config {
            http: Box::new(http),
            matrix: Box::new(matrix),
        }
    }
}

pub fn load_config(path: String) -> Config {
    let cfg_path = match path.as_ref() {
        "" => "cfg.ini",
        x => x,
    };

    let cfg_ini = Ini::load_from_file(cfg_path).unwrap();
    Config::load_from_ini(cfg_ini)
}
