use config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Langs {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub corpus_dir: String,
    pub tok_prefix: String,
    pub langs: Langs,
    pub alignment_file_path: String,
    pub orig_prefix: String,
    pub output_amphigram_path: String,
    pub textunit_limit: usize,
    pub textunit_offset: usize,
}

impl Config {
    #[allow(dead_code)]
    pub fn load() -> Config {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("config")).unwrap();
        let config: Config = settings.try_into::<Config>().unwrap();
        config
    }
}
