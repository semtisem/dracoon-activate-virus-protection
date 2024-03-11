use dco3::{auth::Connected, Dracoon, OAuth2Flow};

use crate::client::config::{AuthConfig, CredentialsAuthCodeFlow, CredentialsPasswordFlow};
use anyhow::Result;

#[derive(Clone)]
pub struct Client {}

impl Client {
    #[allow(unused)]
    pub async fn connect_password_flow() -> Dracoon<Connected> {
        let auth_config = AuthConfig::<CredentialsPasswordFlow>::load_default_config();
        
        let dracoon = Dracoon::builder()
            .with_base_url(auth_config.base_url.clone())
            .with_client_id(auth_config.client_id.clone())
            .with_client_secret(auth_config.client_secret.clone())
            .build()
            .unwrap();

        dracoon.connect(OAuth2Flow::PasswordFlow(auth_config.username.clone(), auth_config.password.clone()))
            .await.unwrap()
    }

    #[allow(unused)]
    pub async fn connect_auth_code_flow() -> Dracoon<Connected> {
        let auth_config = AuthConfig::<CredentialsAuthCodeFlow>::load_default_config();

        let dracoon = Dracoon::builder()
            .with_base_url(auth_config.base_url.clone())
            .with_client_id(auth_config.client_id.clone())
            .with_client_secret(auth_config.client_secret.clone())
            .with_redirect_uri(auth_config.redirect_uri.clone())
            .build()
            .unwrap();        

        println!("Please log in via browser (open url): ");
        println!("{}", dracoon.get_authorize_url());

        let auth_code = dialoguer::Password::new()
            .with_prompt("Please enter authorization code")
            .interact()
            .unwrap();

        dracoon.connect(OAuth2Flow::AuthCodeFlow(auth_code.trim_end().into()))
            .await.unwrap()
    }
}