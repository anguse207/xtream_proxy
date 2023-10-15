use serde::{Deserialize, Serialize};
use std::fs;
use std::process;
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

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Config {
//     pub host: HashMap<String, String>,
//     pub xtream: HashMap<String, String>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub host: Host,
    pub xtream: Xtream,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    pub address: String,
    pub user: String,
    pub pass: String,
    pub user_agent: String,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Xtream {
    pub address: String,
    pub user: String,
    pub pass: String,
}

impl Config {
    fn create_default() -> Self {
        let host: Host = Host {
            address: HOST_ADDR.to_owned(),
            user: HOST_USER.to_owned(),
            pass: HOST_PASS.to_owned(),
            user_agent: USER_AGENT.to_owned(),
            timeout: 300,
        };

        let xtream: Xtream = Xtream {
            address: XT_ADDR.to_owned(),
            user: XT_USER.to_owned(),
            pass: XT_PASS.to_owned(),
        };

        let default = Config { host, xtream };

        let serialized_data = toml::to_string(&default).unwrap();

        let write_handle = fs::write(CONFIG, serialized_data);

        match write_handle {
            Ok(_) => {
                println!("Created new {CONFIG}\nEdit your {CONFIG} and then relaunch");
                process::exit(0x0100);
            }
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
