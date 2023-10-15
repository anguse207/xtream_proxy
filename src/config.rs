use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

const CONFIG: &str = "Config.toml";

//DEFAULTS
// HOST
const USER_AGENT: &str = "tivimate";
const HOST_ADDR: &str = "address:port";
const HOST_USER: &str = "user";
const HOST_PASS: &str = "pass";

// xtream / TARGET
const XT_ADDR: &str = "address:port";
const XT_USER: &str = "user";
const XT_PASS: &str = "pass";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub host: HashMap<String, String>,
    pub xtream: HashMap<String, String>,
}

impl Config {
    fn create_default() -> Self {
        let mut host: HashMap<String, String> = HashMap::new();
        host.insert("user_agent".to_owned(), USER_AGENT.to_owned());
        host.insert("address".to_owned(), HOST_ADDR.to_owned());
        host.insert("user".to_owned(), HOST_USER.to_owned());
        host.insert("pass".to_owned(), HOST_PASS.to_owned());

        let mut xtream: HashMap<String, String> = HashMap::new();
        xtream.insert("address".to_owned(), XT_ADDR.to_owned());
        xtream.insert("user".to_owned(), XT_USER.to_owned());
        xtream.insert("pass".to_owned(), XT_PASS.to_owned());

        let default = Config { host, xtream };

        let data = toml::to_string(&default).unwrap();

        let write_handle = fs::write(CONFIG, data);

        match write_handle {
            Ok(_) => println!("Created new {CONFIG}, with defaults"),
            Err(e) => println!("Could NOT create {CONFIG}. \n{e}"),
        }

        default
    }
    pub fn new() -> Self {
        let read_handle = fs::read_to_string(CONFIG);
        let config: Config;

        match read_handle {
            Ok(data) => {
                config = Self::load(&data);
                println!("Existing {CONFIG} found... Loaded...\n{:#?}", &config);
            }
            Err(e) => {
                match e.kind() {
                    //
                    std::io::ErrorKind::NotFound => {
                        println!("No {CONFIG} found...");
                        config = Self::create_default();
                    }
                    _ => {
                        println!("Unhandled Error: !{e}");
                        config = Self::create_default();
                    }
                }
            }
        }

        config
    }

    fn load(data: &str) -> Self {
        let config = toml::from_str(data);

        match config {
            Ok(d) => d,
            Err(e) => {
                println!("Unhandled Error: !{e}");
                Self::create_default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
