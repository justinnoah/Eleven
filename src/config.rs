/// cfg.ini Layout
///
/// [http]
/// address = "0.0.0.0:8448"
/// key_path = "tls/"

use std::collections::{HashMap};

use ini::Ini;

pub struct Config {
    pub http: Box<HashMap<String, String>>,
}

impl Config {
    pub fn load_from_ini(cfg: Ini) -> Config {
        let http = cfg.section(Some("http")).unwrap().clone();
        Config {
            http: Box::new(http),
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
