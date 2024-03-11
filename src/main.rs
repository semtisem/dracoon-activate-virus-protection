use client::Client;
use dco3::User;

mod client;


#[tokio::main]
async fn main() {
    let dracoon = Client::connect_auth_code_flow().await;

    let current_user = dracoon.get_user_account().await.unwrap();
    print!("{:?}", current_user);
}
