use anyhow::Result;
use aws_sdk_dynamodb::types::AttributeValue;
use owen::{constants::MAX_DDEX_PER_BLOB, output_generator::MessageDirProcessingContext};
use std::{collections::HashMap, env};

pub struct MessageQueue {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
    index_name: String,
    pk_name: String,
    processing_status_attribute_name: String,
    pub unprocessed_status_value: String,
    pub processed_status_value: String,
    pub reserved_status_value: String,
    pub rejected_status_value: String,
    owen_instance_attribute_name: String,
    owen_instance_name: String,
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
            processing_status_attribute_name: MessageQueue::get_env_var(
                "PROCESSING_STATUS_ATTRIBUTE_NAME",
            ),
            owen_instance_attribute_name: MessageQueue::get_env_var("OWEN_INSTANCE_ATTRIBUTE_NAME"),
            unprocessed_status_value: MessageQueue::get_env_var("UNPROCESSED_STATUS_VALUE"),
            processed_status_value: MessageQueue::get_env_var("PROCESSED_STATUS_VALUE"),
            reserved_status_value: MessageQueue::get_env_var("RESERVED_STATUS_VALUE"),
            rejected_status_value: MessageQueue::get_env_var("REJECTED_STATUS_VALUE"),
            owen_instance_name: MessageQueue::get_env_var("USERNAME"),
        }
    }

    pub async fn get_message_folders(&self) -> Result<Vec<String>> {
        let response = self
            .client
            .query()
            .table_name(&self.table_name)
            .index_name(&self.index_name)
            .key_condition_expression(format!(
                "{} = :expressionValue",
                &self.processing_status_attribute_name
            ))
            .expression_attribute_values(
                ":expressionValue",
                AttributeValue::S(self.unprocessed_status_value.to_string()),
            )
            .limit(MAX_DDEX_PER_BLOB)
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

    async fn set_single_message_folder_status(
        &self,
        message_folder: String,
        status: String,
    ) -> Result<()> {
        let folder_key = AttributeValue::S(message_folder);
        let status_value = AttributeValue::S(status);
        let owen_instance_value = AttributeValue::S(self.owen_instance_name.clone());

        let update_output = &self
            .client
            .update_item()
            .table_name(&self.table_name)
            .key("messageFolder", folder_key)
            .update_expression(format!(
                "SET {} = :statusValue, {} = :owenInstanceValue",
                &self.processing_status_attribute_name, &self.owen_instance_attribute_name
            ))
            .expression_attribute_values(":statusValue", status_value)
            .expression_attribute_values(":owenInstanceValue", owen_instance_value)
            .send()
            .await?;
        println!("update output: {update_output:?}");
        Ok(())
    }

    pub async fn set_message_folders_status(
        &self,
        message_folders: &Vec<String>,
        status: String,
    ) -> Result<()> {
        for folder in message_folders {
            self.set_single_message_folder_status(folder.clone(), status.clone())
                .await?;
        }
        Ok(())
    }

    pub async fn sync_message_folder_statuses(
        &self,
        local_to_s3_folder_mapping: HashMap<String, String>,
        message_processing_context_vec: Vec<MessageDirProcessingContext>,
        message_folders: Vec<String>,
    ) -> Result<()> {
        let mut s3_folder_to_processing_context_map: HashMap<String, MessageDirProcessingContext> =
            HashMap::new();

        for message_processing_context in message_processing_context_vec {
            let s3_path = local_to_s3_folder_mapping
                .get(&message_processing_context.message_dir_path)
                .expect(
                    format!(
                        "Could not retrieve s3 path from mapping to local folder. Local folder: {}",
                        message_processing_context.message_dir_path
                    )
                    .as_str(),
                );

            s3_folder_to_processing_context_map.insert(s3_path.clone(), message_processing_context);
        }
        for folder in message_folders {
            let message_processing_context = s3_folder_to_processing_context_map.get(&folder);
            if let Some(message_processing_context) = message_processing_context {
                if message_processing_context.excluded {
                    self.set_single_message_folder_status(
                        folder.clone(),
                        self.rejected_status_value.to_string(),
                    )
                    .await?;
                } else {
                    self.set_single_message_folder_status(
                        folder.clone(),
                        self.processed_status_value.to_string(),
                    )
                    .await?;
                }
            } else {
                self.set_single_message_folder_status(
                    folder.clone(),
                    self.rejected_status_value.to_string(),
                )
                .await?;
            }
        }
        Ok(())
    }
}
