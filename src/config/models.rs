use std::{marker::PhantomData, sync::OnceLock};
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CredentialsAuthCodeFlow {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl Credentials for CredentialsAuthCodeFlow {
    fn init(path: Option<String>) -> CredentialsAuthCodeFlow {
        static CONFIG:  OnceLock<CredentialsAuthCodeFlow> = OnceLock::new();
        let relative_path = path.unwrap_or("config.yml".to_string());

        let mut path = process_path::get_executable_path().unwrap();

        path  = path.parent().unwrap().to_path_buf();
        path.push(relative_path);

        print!("{:?}", path.to_str().unwrap().to_string());
        
        let auth_conig = CONFIG.get_or_init(|| {
            Config::builder()
            .add_source(File::new(&path.to_str().unwrap().to_string(), FileFormat::Yaml).required(true))
            .build()
            .expect("Failed to build config")
            .try_deserialize::<CredentialsAuthCodeFlow>()
            .expect("Failed to deserialize config")
        });
        auth_conig.clone()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CredentialsPasswordFlow {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

impl Credentials for CredentialsPasswordFlow {
    fn init(path: Option<String>) -> CredentialsPasswordFlow{
        static CONFIG:  OnceLock<CredentialsPasswordFlow> = OnceLock::new();
        let relative_path = path.unwrap_or("config.yml".to_string());

        let mut path = process_path::get_executable_path().unwrap();

        path  = path.parent().unwrap().to_path_buf();
        path.push(relative_path);

        print!("{:?}", path.to_str().unwrap().to_string());
        let auth_config = CONFIG.get_or_init(|| {
            Config::builder()
            .add_source(File::new(&path.to_str().unwrap().to_string(), FileFormat::Yaml).required(true))
            .build()
            .expect("Failed to build config")
            .try_deserialize::<CredentialsPasswordFlow>()
            .expect("Failed to deserialize config")
        });

        auth_config.clone()
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct AuthConfig<T: Credentials> {
    credentials: PhantomData<T>,
}

pub trait Credentials {
    fn init(path: Option<String>) -> Self;
}

impl<T> AuthConfig<T>
where
    T: Credentials
    {
 
    #[allow(dead_code)]
    pub fn load_config_from_path(path: String) -> T {
        let config = Credentials::init(path.into());
        config
        // TODO validate base_url
    }

    #[allow(dead_code)]
    pub fn load_default_config() -> T {
        let config = Credentials::init(None);
        config
        // TODO validate base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_password_flow() {
        let config = AuthConfig::<CredentialsPasswordFlow>::load_config_from_path("config.passoword.flow.example.yml".to_string());
        assert_eq!(config.client_id, "xxx");
        assert_eq!(config.client_secret,"xxx");
        assert_eq!(config.base_url,"https://dracoon.team");
        assert_eq!(config.username,"username");
        assert_eq!(config.password,"password");
    }

    #[test]
    fn test_load_config_auth_flow() {
        let config = AuthConfig::<CredentialsAuthCodeFlow>::load_config_from_path("config.auth.flow.example.yml".to_string());
        assert_eq!(config.client_id, "xxx");
        assert_eq!(config.client_secret, "xxx");
        assert_eq!(config.base_url, "https://dracoon.team");
    }
}