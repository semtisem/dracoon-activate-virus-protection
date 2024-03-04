use client::Client;


mod client;
mod config;

#[tokio::main]
async fn main() {
    Client::connect_auth_code_flow().await;
}
