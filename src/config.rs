use std::net::SocketAddr;
use std::str::FromStr;

use ini::Ini;


pub struct Config {
    pub address: SocketAddr,
}

impl Config {
    pub fn load_from_ini(cfg: Ini) -> Config {
        let http_addr = SocketAddr::from_str(
            cfg.get_from_or(Some("http"), "address", "0.0.0.0:8448")
        ).unwrap();

        Config {
            address: http_addr,
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
