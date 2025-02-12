use std::{env, error::Error};

#[derive(Debug)]
pub struct Config {
    pub message_status_table_name: String,
    pub unprocessed_status_value: String,
    pub created_timestamp_attribute_name: String,
    pub updated_timestamp_attribute_name: String,
    pub message_folder_attribute_name: String,
    pub processing_status_attribute_name: String,
}

impl Config {
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build() -> Config {
        let message_status_table_name = Config::get_env_var("MESSAGE_STATUS_TABLE_NAME");
        let unprocessed_status_value = Config::get_env_var("UNPROCESSED_STATUS_VALUE");
        let created_timestamp_attribute_name =
            Config::get_env_var("CREATED_TIMESTAMP_ATTRIBUTE_NAME");
        let updated_timestamp_attribute_name =
            Config::get_env_var("UPDATED_TIMESTAMP_ATTRIBUTE_NAME");
        let message_folder_attribute_name = Config::get_env_var("MESSAGE_FOLDER_ATTRIBUTE_NAME");
        let processing_status_attribute_name =
            Config::get_env_var("PROCESSING_STATUS_ATTRIBUTE_NAME");

        Config {
            message_status_table_name,
            unprocessed_status_value,
            created_timestamp_attribute_name,
            updated_timestamp_attribute_name,
            message_folder_attribute_name,
            processing_status_attribute_name,
        }
    }
}
