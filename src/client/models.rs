use dco3::{Dracoon, OAuth2Flow, User};
use crate::config::{ScriptConfigAuthCodeFlow, ScriptConfigPasswordFlow};

#[derive(Clone)]
pub struct Client {
    dracoon: Dracoon,
}

impl Client {
    #[allow(unused)]
    pub fn connect_password_flow() {
        let config = ScriptConfigPasswordFlow::load_default_config();
        print!("{:?}", config.credentials().client_id);
    }

    #[allow(unused)]
    pub async fn connect_auth_code_flow() {
        let config = ScriptConfigAuthCodeFlow::load_default_config();
        let credentials = config.credentials();
        let dracoon = Dracoon::builder()
            .with_base_url(&credentials.base_url)
            .with_client_id(&credentials.client_id)
            .with_client_secret(&credentials.client_secret)
            .with_redirect_uri(&credentials.redirect_uri)
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