use tracing::level_filters::LevelFilter;

use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptConfig {
    dracoon: DracoonConfig,
    logging: LoggingConfig,
    activate_virus_protection: Vec<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    log_level: DebugLevel,
    log_file: Option<LogFileConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LogFileConfig {
    log_file_path: String,
}

impl LogFileConfig {
    pub fn get_log_file_path(&self) -> &String {
        &self.log_file_path
    }
}

impl LoggingConfig {
    pub fn get_log_level(&self) -> LevelFilter {
        match self.log_level {
            DebugLevel::Debug => LevelFilter::DEBUG,
            DebugLevel::Info => LevelFilter::INFO,
            DebugLevel::Warn => LevelFilter::WARN,
            DebugLevel::Error => LevelFilter::ERROR,
        }
    }

    pub fn get_log_file_path(&self) -> &String {
        self.log_file.as_ref().unwrap().get_log_file_path()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DracoonConfig {
    base_url: String,
}

impl DracoonConfig {
    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum DebugLevel {
    #[serde(rename = "debug")]
    Debug,
    /// The "info" level.
    ///
    /// Designates useful information.
    #[serde(rename = "info")]
    Info,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    #[serde(rename = "warn")]
    Warn,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    #[serde(rename = "error")]
    Error,
}

impl ScriptConfig {
    pub fn init(path: Option<String>) -> &'static ScriptConfig {
        static CONFIG: OnceLock<ScriptConfig> = OnceLock::new();
        let relative_path = path.unwrap_or("./config.yml".to_string());

        let mut path = process_path::get_executable_path().expect("Failed to get executable path.");
        path = path.parent().unwrap().to_path_buf();
        path.push(relative_path);

        let config = CONFIG.get_or_init(|| {
            Config::builder()
                .add_source(File::new(path.to_str().unwrap(), FileFormat::Yaml).required(true))
                .build()
                .map_err(|e| {
                    println!("Couldn't find a config file. {}", e);
                })
                .unwrap()
                .try_deserialize::<ScriptConfig>()
                .map_err(|e| {
                    println!("Failed to deserialize config: {}", e);
                })
                .unwrap()
        });
        config
    }

    pub fn get_logging_config(&self) -> &LoggingConfig {
        &self.logging
    }

    pub fn get_virus_protection_rooms(&self) -> &Vec<u64> {
        &self.activate_virus_protection
    }

    pub fn get_dracoon_config(&self) -> &DracoonConfig {
        &self.dracoon
    }
}
