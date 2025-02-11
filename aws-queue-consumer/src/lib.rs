use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub message_status_table_name: String,
}

impl Config {
    fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        match env::var(key) {
            Ok(value) => Ok(value),
            Err(err) => return Err(Box::new(err)),
        }
    }
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let message_status_table_name = Config::get_env_var("MESSAGE_STATUS_TABLE_NAME")?;
        Ok(Config {
            message_status_table_name,
        })
    }
}
