use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Ebisu, telegram alert service adapter, written in Rust !")]
pub struct Options {
    /// Ebisu config path
    #[structopt(long = "config.ebisu", default_value = "config/ebisu.yaml")]
    config: String,
}

impl Options {
    pub fn new() -> Self {
        Options::from_args()
    }

    pub fn get_config_path(&self) -> &str {
        self.config.as_str()
    }
}
