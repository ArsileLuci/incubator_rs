use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Request {
    #[serde(rename = "type")]
    _type: String,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: DebugInfo,
}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DebugInfo {
    #[serde(with = "humantime_serde")]
    duration: Duration, //
    at: chrono::DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Gift {
    id: u64,
    price: u64,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Stream {
    user_id: String, //GUID
    is_private: bool,
    settings: u32,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct PublicTariff {
    id: u64,
    price: u64,
    #[serde(with = "humantime_serde")]
    duration: Duration, //Duration from string
    description: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct PrivateTariff {
    client_price: u64,
    #[serde(with = "humantime_serde")]
    duration: Duration, //
    description: String,
}

fn main() {
    let r: Request =
        serde_json::from_reader(std::fs::File::open("./request.json").unwrap()).unwrap();
    serde_yaml::to_writer(std::fs::File::create("./request.yaml").unwrap(), &r);
    let mut toml = std::fs::File::create("./request.toml").unwrap();
    let t = toml::to_string(&r).unwrap();
    toml.write(t.as_bytes());
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn test_json_to_yaml() {
        let json: Request =
            serde_json::from_reader(std::fs::File::open("./request.json").unwrap()).unwrap();
        serde_yaml::to_writer(std::fs::File::create("./request.yaml").unwrap(), &json);
        let yaml: Request =
            serde_yaml::from_reader(std::fs::File::open("./request.yaml").unwrap()).unwrap();
        assert_eq!(yaml, json);
    }

    #[test]
    fn test_json_to_toml() {
        let json: Request =
            serde_json::from_reader(std::fs::File::open("./request.json").unwrap()).unwrap();
        let t = toml::to_string(&json).unwrap();
        let toml: Request = toml::from_str(&t).unwrap();
        assert_eq!(toml, json);
    }
}
