use {getset::Getters, log::info, serde::Deserialize};
use std::time::Duration;

#[derive(Deserialize, Getters, Clone)]
pub struct TelegramConfig {
    #[getset(get = "pub with_prefix")]
    chat_id: String,
    #[getset(get = "pub with_prefix")]
    token: String,
}


#[derive(Deserialize, Getters)]
pub struct ServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u16,
    #[getset(get = "pub with_prefix")]
    buffer: usize,
    #[getset(get = "pub with_prefix")]
    concurrency_limit: usize,
    #[getset(get = "pub with_prefix")]
    rate_limit: u64,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    limit_timeout: Duration
}

#[derive(Deserialize, Getters)]
pub struct Config {
    pub telegram: TelegramConfig,
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
}

fn default_server_config() -> ServerConfig {
    ServerConfig { port: 8080, buffer: 10, concurrency_limit: 5, rate_limit: 5,  limit_timeout: Duration::from_secs(10)}
}

pub fn read_config(config_path: &str) -> Config {
    let f = std::fs::File::open(config_path).expect("Couldn't found file");
    let config: Config = serde_yaml::from_reader(f).expect("Parse failed");
    info!("Config has been read from config in : {}", config_path);
    config
}
