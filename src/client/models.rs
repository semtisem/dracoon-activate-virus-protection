use dco3::{auth::Connected, Dracoon, OAuth2Flow};

#[derive(Clone)]
pub struct Client {}

impl Client {
    #[allow(unused)]
    pub async fn connect_password_flow(base_url: &String, client_id: String, client_secret: String) -> Dracoon<Connected> {
        let dracoon = Dracoon::builder()
            .with_base_url(base_url)
            .with_client_id(client_id)
            .with_client_secret(client_secret)
            .build()
            .unwrap();

        let username = "test".to_string();
        let password = "test".to_string();

        dracoon.connect(OAuth2Flow::PasswordFlow(username, password))
            .await.unwrap()
    }

    #[allow(unused)]
    pub async fn connect_auth_code_flow(base_url: &String, client_id: String, client_secret: String) -> Dracoon<Connected> {
        let dracoon = Dracoon::builder()
            .with_base_url(base_url)
            .with_client_id(client_id)
            .with_client_secret(client_secret)
            .with_redirect_uri(base_url.to_owned() + "/oauth/callback")
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