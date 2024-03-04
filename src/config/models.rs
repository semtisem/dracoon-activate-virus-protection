use std::sync::OnceLock;
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CredentialsAuthCodeFlow {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CredentialsPasswordFlow {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptConfigAuthCodeFlow {
    credentials: CredentialsAuthCodeFlow,
}

impl ScriptConfigAuthCodeFlow {
    pub fn credentials(&self) -> &CredentialsAuthCodeFlow {
        &self.credentials
    }

    #[allow(dead_code)]
    pub fn load_config_from_path<'a>(path: String) -> &'a ScriptConfigAuthCodeFlow {
        let config = init_config(path.into());
        config
        // TODO validate base_url
        
    }

    #[allow(dead_code)]
    pub fn load_default_config<'a>() -> &'a ScriptConfigAuthCodeFlow {
        let config = init_config(None);
        config

        // TODO validate base_url
        
    }
}


fn init_config<'a>(path: Option<String>) -> &'a ScriptConfigAuthCodeFlow {
    static CONFIG:  OnceLock<ScriptConfigAuthCodeFlow> = OnceLock::new();
    let relative_path = path.unwrap_or("config.yml".to_string());

    let mut path = process_path::get_executable_path().unwrap();

    path  = path.parent().unwrap().to_path_buf();
    path.push(relative_path);

    print!("{:?}", path.to_str().unwrap().to_string());
    CONFIG.get_or_init(|| {
        Config::builder()
        .add_source(File::new(&path.to_str().unwrap().to_string(), FileFormat::Yaml).required(true))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<ScriptConfigAuthCodeFlow>()
        .expect("Failed to deserialize config")
    })
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
pub struct ScriptConfigPasswordFlow {
    credentials: CredentialsPasswordFlow ,
}

#[allow(unused)]
impl ScriptConfigPasswordFlow {
    pub fn credentials(&self) -> &CredentialsPasswordFlow {
        &self.credentials
    }

    #[allow(dead_code)]
    pub fn load_config_from_path<'a>(path: String) -> &'a ScriptConfigPasswordFlow {
        let config = init_config_password_flow(path.into());
        config
        // TODO validate base_url
        
    }

    #[allow(dead_code)]
    pub fn load_default_config<'a>() -> &'a ScriptConfigPasswordFlow {
        let config = init_config_password_flow(None);
        config

        // TODO validate base_url
        
    }
}


fn init_config_password_flow<'a>(path: Option<String>) -> &'a ScriptConfigPasswordFlow {
    static CONFIG:  OnceLock<ScriptConfigPasswordFlow> = OnceLock::new();

    let relative_path = path.unwrap_or("config.yml".to_string());

    let mut path = process_path::get_executable_path().unwrap();
    path.push(relative_path);

    print!("{:?}", path.to_str().unwrap().to_string());

    CONFIG.get_or_init(|| {
        Config::builder()
        .add_source(File::new(&path.to_str().unwrap().to_string(), FileFormat::Yaml).required(true))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<ScriptConfigPasswordFlow>()
        .expect("Failed to deserialize config")
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = ScriptConfigAuthCodeFlow::load_config_from_path("config.example.yml".to_string());
        assert_eq!(config.credentials().client_id, "xxx");
        assert_eq!(
            config.credentials().client_secret,
            "xxx"
        );
        assert_eq!(
            config.credentials().base_url,
            "https://dracoon.team"
        );
    }
}