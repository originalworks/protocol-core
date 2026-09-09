use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub message_status_table_name: String,
    pub processing_status_index_name: String,
    pub message_folder_attribute_name: String,
    pub processing_status_attribute_name: String,
    pub created_timestamp_attribute_name: String,
    pub updated_timestamp_attribute_name: String,
    pub owen_instance_attribute_name: String,
}

impl Config {
    fn get_env_var(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Missing env variable: {key}"))
    }

    fn get_env_var_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    pub fn build() -> Config {
        Config {
            message_status_table_name: Config::get_env_var("MESSAGE_STATUS_TABLE_NAME"),
            processing_status_index_name: Config::get_env_var("PROCESSING_STATUS_INDEX_NAME"),
            message_folder_attribute_name: Config::get_env_var_or(
                "MESSAGE_FOLDER_ATTRIBUTE_NAME",
                "messageFolder",
            ),
            processing_status_attribute_name: Config::get_env_var_or(
                "PROCESSING_STATUS_ATTRIBUTE_NAME",
                "processingStatus",
            ),
            created_timestamp_attribute_name: Config::get_env_var_or(
                "CREATED_TIMESTAMP_ATTRIBUTE_NAME",
                "createdTimestamp",
            ),
            updated_timestamp_attribute_name: Config::get_env_var_or(
                "UPDATED_TIMESTAMP_ATTRIBUTE_NAME",
                "updatedTimestamp",
            ),
            owen_instance_attribute_name: Config::get_env_var_or(
                "OWEN_INSTANCE_ATTRIBUTE_NAME",
                "owenInstance",
            ),
        }
    }
}
