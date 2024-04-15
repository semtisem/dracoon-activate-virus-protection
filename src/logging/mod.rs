use std::fs::OpenOptions;

use tracing_subscriber::filter::EnvFilter;

use crate::config::LoggingConfig;

pub struct Logging;

impl Logging {
    pub fn setup(logging_config: &LoggingConfig) { 

        let env_filter =  EnvFilter::from_default_env().add_directive(logging_config.get_log_level().into());

         // set up logging format
         let log_format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_thread_names(false)
            .with_target(true)
            .with_ansi(false)
            .with_file(true)
            .with_line_number(true)
            .with_source_location(true)
            .compact();

        
            let log_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(logging_config.get_log_file_path())
                .map_err(|e| {
                    println!("Failed to create or open log file: {}", e);
                }).unwrap();
           
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .event_format(log_format)
                .with_writer(std::sync::Mutex::new(log_file))
                .init()
            
    }
}
