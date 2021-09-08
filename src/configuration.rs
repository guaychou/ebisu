use {getset::Getters, log::info, serde::Deserialize};

#[derive(Deserialize, Getters, Clone)]
pub struct TelegramConfig {
    #[getset(get = "pub with_prefix")]
    chat_id: String,
    #[getset(get = "pub with_prefix")]
    token: String,
}

#[derive(Deserialize, Getters, Clone)]
pub struct DiscordConfig {
    #[getset(get = "pub with_prefix")]
    webhook_url: String,
}

#[derive(Deserialize, Getters)]
pub struct ServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u16,
}

#[derive(Deserialize, Getters)]
pub struct Config {
    pub telegram: TelegramConfig,
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
}

fn default_server_config() -> ServerConfig {
    ServerConfig { port: 8080 }
}

pub fn read_config(config_path: &str) -> Config {
    let f = std::fs::File::open(config_path).expect("Couldn't found file");
    let config: Config = serde_yaml::from_reader(f).expect("Parse failed");
    info!("Config has been read from config in : {}", config_path);
    config
}
