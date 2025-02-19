use aws_sdk_dynamodb::types::AttributeValue;
use owen_cli::output_generator::MessageDirProcessingContext;
use std::{collections::HashMap, env, error::Error};

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
            .expect("No items in Dynamo response")
            .iter()
            .map(|item| {
                item.get(&self.pk_name)
                    .expect("Could not find partition key value")
                    .as_s()
                    .expect("Partition key is not a string")
                    .clone()
            })
            .collect::<Vec<String>>();

        Ok(message_folders)
    }

    pub async fn set_message_folders_as_processed(
        &self,
        message_folders: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        for folder in message_folders {
            let folder_key = AttributeValue::S(folder.clone());
            let status_value = AttributeValue::S(self.processed_status_value.to_string().clone());

            let update_output = &self
                .client
                .update_item()
                .table_name(&self.table_name)
                .key("messageFolder", folder_key)
                .update_expression("SET processingStatus = :expressionValue")
                .expression_attribute_values(":expressionValue", status_value)
                .send()
                .await?;
            println!("update output: {update_output:?}");
        }
        Ok(())
    }
    pub async fn set_message_folders_as_rejected(
        &self,
        message_folders: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        for folder in message_folders {
            let folder_key = AttributeValue::S(folder.clone());
            let status_value = AttributeValue::S("rejected".to_string());

            let update_output = &self
                .client
                .update_item()
                .table_name(&self.table_name)
                .key("messageFolder", folder_key)
                .update_expression("SET processingStatus = :expressionValue")
                .expression_attribute_values(":expressionValue", status_value)
                .send()
                .await?;
            println!("update output: {update_output:?}");
        }
        Ok(())
    }
    pub async fn sync_message_folder_statuses(
        &self,
        local_to_s3_folder_mapping: HashMap<String, String>,
        message_processing_context_vec: Vec<MessageDirProcessingContext>,
        message_folders: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut s3_folder_to_processing_context_map: HashMap<String, MessageDirProcessingContext> =
            HashMap::new();

        for message_processing_context in message_processing_context_vec {
            let s3_path = local_to_s3_folder_mapping
                .get(&message_processing_context.message_dir_path)
                .unwrap();

            s3_folder_to_processing_context_map.insert(s3_path.clone(), message_processing_context);
        }
        for folder in message_folders {
            let folder_key = AttributeValue::S(folder.clone());
            let message_processing_context = s3_folder_to_processing_context_map.get(&folder);
            if let Some(message_processing_context) = message_processing_context {
                if message_processing_context.excluded {
                    let update_output = &self
                        .client
                        .update_item()
                        .table_name(&self.table_name)
                        .key("messageFolder", folder_key)
                        .update_expression("SET processingStatus = :expressionValue")
                        .expression_attribute_values(
                            ":expressionValue",
                            AttributeValue::S("rejected".to_string()),
                        )
                        .send()
                        .await?;
                    println!("update output: {update_output:?}");
                } else {
                    let update_output = &self
                        .client
                        .update_item()
                        .table_name(&self.table_name)
                        .key("messageFolder", folder_key)
                        .update_expression("SET processingStatus = :expressionValue")
                        .expression_attribute_values(
                            ":expressionValue",
                            AttributeValue::S("processed".to_string()),
                        )
                        .send()
                        .await?;
                    println!("update output: {update_output:?}");
                }
            } else {
                let update_output = &self
                    .client
                    .update_item()
                    .table_name(&self.table_name)
                    .key("messageFolder", folder_key)
                    .update_expression("SET processingStatus = :expressionValue")
                    .expression_attribute_values(
                        ":expressionValue",
                        AttributeValue::S("rejected".to_string()),
                    )
                    .send()
                    .await?;
                println!("update output: {update_output:?}");
            }
        }
        Ok(())
    }
}
