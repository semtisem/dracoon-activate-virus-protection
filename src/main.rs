use client::Client;
use dco3::User;
use tracing::{error, info};

use crate::config::ScriptConfig;
use crate::logging::Logging;

mod client;
mod config;
mod logging;

#[tokio::main]
async fn main() {
    let config = ScriptConfig::init(None);

    Logging::setup(config.get_logging_config());

    error!("This is an error message");
    info!("This is an info message");

    let dracoon = Client::connect_auth_code_flow().await;
 
    let current_user = dracoon.get_user_account().await.unwrap();
    print!("{:?}", current_user);
}
