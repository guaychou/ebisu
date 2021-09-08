use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Ebisu, telegram alert service adapter, written in Rust !")]
pub struct Options {
    /// Ebisu config path
    #[structopt(long = "config.ebisu", default_value = "config/ebisu.yaml")]
    config: String,
    /// log4rs config path
    #[structopt(long = "config.log", default_value = "config/log4rs.yaml")]
    log_config: String,
}

impl Options {
    pub fn new() -> Self {
        Options::from_args()
    }

    pub fn get_config_path(&self) -> &str {
        self.config.as_str()
    }

    pub fn get_log_config_path(&self) -> &str {
        self.log_config.as_str()
    }
}
