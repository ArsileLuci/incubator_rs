#[macro_use]
extern crate structopt;

use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;
fn main() {
    let opt = Opt::from_args();
    let conf_path: String;
    if opt.conf.is_some() {
        conf_path = opt.conf.unwrap();
    } else {
        let path = std::env::vars().filter(|x| x.0 == "CONF_FILE").next();
        if path.is_some() {
            conf_path = path.unwrap().1.clone();
        } else {
            conf_path = "config.toml".to_owned();
        }
    }
    println!("{}", conf_path);
    let mut c = config::Config::new();

    c.merge(config::File::with_name(&conf_path).required(false));
    c.merge(config::Environment::with_prefix("conf").separator("_"))
        .unwrap();
    let config: Config = c.try_into().unwrap();
    println!("{:?}", config);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "3_9", about = "Prints its configuration to STDOUT.")]
struct Opt {
    ///Enables debug mode
    #[structopt(short, long)]
    debug: bool,
    ///Prints version information
    #[structopt(short, long)]
    version: bool,
    ///Path to configuration file [env: CONF_FILE=]  [default: config.toml]
    #[structopt(short, long)]
    pub conf: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Config {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Mode {
    debug: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Server {
    exernal_url: String,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Db {
    mysql: Mysql,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Mysql {
    host: String,
    port: u16,
    dating: String,
    user: String,
    pass: String,
    connections: Connections,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Connections {
    max_idle: u32,
    max_open: u32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Log {
    app: App,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct App {
    level: Level,
}

#[derive(Debug, Deserialize)]
enum Level {
    error,
    warn,
    info,
    debug,
    trace,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Background {
    watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Watchdog {
    #[serde(with = "humantime_serde")]
    period: std::time::Duration,
    limit: u16,
    #[serde(with = "humantime_serde")]
    lock_timeout: std::time::Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: Mode::default(),
            server: Server::default(),
            db: Db::default(),
            log: Log::default(),
            background: Background::default(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode { debug: false }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            exernal_url: "http://127.0.0.1".to_owned(),
            http_port: 8081,
            grpc_port: 8082,
            healthz_port: 10025,
            metrics_port: 9199,
        }
    }
}

impl Default for Db {
    fn default() -> Self {
        Db {
            mysql: Mysql::default(),
        }
    }
}

impl Default for Mysql {
    fn default() -> Self {
        Mysql {
            host: "127.0.0.1".to_owned(),
            port: 3306,
            dating: "default".to_owned(),
            user: "root".to_owned(),
            pass: "".to_owned(),
            connections: Connections::default(),
        }
    }
}

impl Default for Connections {
    fn default() -> Self {
        Connections {
            max_idle: 30,
            max_open: 30,
        }
    }
}

impl Default for Log {
    fn default() -> Self {
        Log {
            app: App::default(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App { level: Level::info }
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            watchdog: Watchdog::default(),
        }
    }
}

impl Default for Watchdog {
    fn default() -> Self {
        Watchdog {
            period: std::time::Duration::from_secs(5),
            limit: 10,
            lock_timeout: std::time::Duration::from_secs(4),
        }
    }
}
