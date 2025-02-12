use aws_sdk_dynamodb::types::AttributeValue;
use std::{env, error::Error};

// use crate::lambda_config::OwenLambdaConfig;

pub struct MessageQueue {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
    index_name: String,
    pk_name: String,
    index_attribute_name: String,
    unprocessed_status_value: String,
    processed_status_value: String,
    messages_per_blob: String,
}

impl MessageQueue {
    pub fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build(aws_main_config: &aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_dynamodb::Client::new(aws_main_config),
            table_name: MessageQueue::get_env_var("MESSAGE_STATUS_TABLE_NAME"),
            index_name: MessageQueue::get_env_var("PROCESSING_STATUS_INDEX_NAME"),
            pk_name: MessageQueue::get_env_var("MESSAGE_FOLDER_ATTRIBUTE_NAME"),
            index_attribute_name: MessageQueue::get_env_var("PROCESSING_STATUS_ATTRIBUTE_NAME"),
            unprocessed_status_value: MessageQueue::get_env_var("UNPROCESSED_STATUS_VALUE"),
            processed_status_value: MessageQueue::get_env_var("PROCESSED_STATUS_VALUE"),
            messages_per_blob: MessageQueue::get_env_var("MESSAGES_PER_BLOB"),
        }
    }

    pub async fn get_message_folders(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self
            .client
            .query()
            .table_name(&self.table_name)
            .index_name(&self.index_name)
            .key_condition_expression(format!("{} = :expressionValue", &self.index_attribute_name))
            .expression_attribute_values(
                ":expressionValue",
                AttributeValue::S(self.unprocessed_status_value.to_string()),
            )
            .limit(self.messages_per_blob.parse::<i32>()?)
            .scan_index_forward(true) // Ascending order (oldest first)
            .send()
            .await?;

        let message_folders = response
            .items
            .unwrap()
            .iter()
            .map(|item| item.get(&self.pk_name).unwrap().as_s().unwrap().clone())
            .collect::<Vec<String>>();

        Ok(message_folders)
    }
}
