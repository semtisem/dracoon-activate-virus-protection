use dco3::{Dracoon, OAuth2Flow, User};
use crate::config::{AuthConfig, CredentialsAuthCodeFlow, CredentialsPasswordFlow};

#[derive(Clone)]
pub struct Client {
    dracoon: Dracoon,
}

impl Client {
    #[allow(unused)]
    pub fn connect_password_flow() {
        let auth_config = AuthConfig::<CredentialsPasswordFlow>::load_default_config();
        print!("{:?}", auth_config.username);
    }

    #[allow(unused)]
    pub async fn connect_auth_code_flow() {
        let auth_config = AuthConfig::<CredentialsAuthCodeFlow>::load_default_config();

        let dracoon = Dracoon::builder()
            .with_base_url(auth_config.base_url)
            .with_client_id(auth_config.client_id)
            .with_client_secret(auth_config.client_secret)
            .with_redirect_uri(auth_config.redirect_uri)
            .build()
            .unwrap();

       
        println!("Please log in via browser (open url): ");
        println!("{}", dracoon.get_authorize_url());

        let auth_code = dialoguer::Password::new()
            .with_prompt("Please enter authorization code")
            .interact()
            .unwrap();

        let dracoon = dracoon
            .connect(OAuth2Flow::AuthCodeFlow(auth_code.trim_end().into()))
            .await
            .unwrap();

        let user_info = dracoon.get_user_account().await.unwrap();
        println!("User info: {:?}", user_info);
    }
}