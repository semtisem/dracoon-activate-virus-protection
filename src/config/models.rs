use tracing::{debug, error, level_filters::LevelFilter, metadata::Level};

use std::{sync::OnceLock};
use config::{Config, File, FileFormat};
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptConfig {
    logging: LoggingConfig,
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

    pub fn create_log_file(&self) -> bool {
        !self.log_file.is_none()
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
        static CONFIG:  OnceLock<ScriptConfig> = OnceLock::new();
        let relative_path = path.unwrap_or("config.yml".to_string());

        let path = process_path::get_executable_path();

        let mut path = match path {
            Some(p) => p,
            None => {
                error!("Failed to get executable path as a result using default config. Log level is to error. No log file is created.");
                return &CONFIG.get_or_init(|| ScriptConfig {
                    logging: LoggingConfig {
                        log_level: DebugLevel::Error,
                        log_file: None,
                    }
                });
            }
        };
        
        path = path.parent().unwrap().to_path_buf();
        path.push(relative_path);
        
        let config = CONFIG.get_or_init(|| {
            Config::builder()
            .add_source(File::new(&path.to_str().unwrap().to_string(), FileFormat::Yaml).required(true))
            .build()
            .map_err(|e| {
                println!("Failed to build config: {}", e);
            }).unwrap()
            .try_deserialize::<ScriptConfig>()
            .map_err(|e| {
                println!("Failed to deserialize config: {}", e);
            }).unwrap()
        });
        config
    }

    pub fn get_logging_config(&self) -> &LoggingConfig {
        &self.logging
    }
}


#[cfg(test)]
mod tests {
    use super::*;


}