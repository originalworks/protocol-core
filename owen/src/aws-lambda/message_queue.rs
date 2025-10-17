use anyhow::Result;
use aws_sdk_dynamodb::types::AttributeValue;
use owen::output_generator::DdexMessage;
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

    pub async fn reserve_message_folder(&self) -> Result<Option<String>> {
        loop {
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
                .limit(1)
                .scan_index_forward(true) // Ascending order (oldest first)
                .send()
                .await?;

            if let Some(items) = response.items {
                if let Some(item) = items.first() {
                    let message_folder = item
                        .get(&self.pk_name)
                        .expect("Could not find partition key value")
                        .as_s()
                        .expect("Partition key is not a string")
                        .clone();

                    match self.try_reserve(message_folder).await? {
                        Some(message_folder) => return Ok(Some(message_folder)),
                        None => continue,
                    }
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        }
    }

    async fn try_reserve(&self, message_folder: String) -> Result<Option<String>> {
        let folder_key = AttributeValue::S(message_folder.clone());
        let reserved_status_value = AttributeValue::S(self.reserved_status_value.clone());
        let unprocessed_status_value = AttributeValue::S(self.unprocessed_status_value.clone());
        let owen_instance_value = AttributeValue::S(self.owen_instance_name.clone());

        let update_output = &self
            .client
            .update_item()
            .table_name(&self.table_name)
            .key("messageFolder", folder_key)
            .update_expression(format!(
                "SET {} = :reservedStatusValue, {} = :owenInstanceValue",
                &self.processing_status_attribute_name, &self.owen_instance_attribute_name
            ))
            .condition_expression(format!(
                "{} = :unprocessedStatusValue",
                &self.processing_status_attribute_name
            ))
            .expression_attribute_values(":reservedStatusValue", reserved_status_value)
            .expression_attribute_values(":unprocessedStatusValue", unprocessed_status_value)
            .expression_attribute_values(":owenInstanceValue", owen_instance_value)
            .send()
            .await;

        match update_output {
            Ok(_) => {
                println!("Item reserved successfully");
                Ok(Some(message_folder.clone()))
            }
            Err(_err) => Ok(None),
        }
    }

    pub async fn set_single_message_folder_status(
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
        ddex_messages: Vec<DdexMessage>,
        message_folders: Vec<String>,
    ) -> Result<()> {
        let mut s3_folder_to_ddex_message_map: HashMap<String, DdexMessage> = HashMap::new();

        for ddex_message in ddex_messages {
            let s3_path = local_to_s3_folder_mapping
                .get(&ddex_message.message_dir_path)
                .expect(
                    format!(
                        "Could not retrieve s3 path from mapping to local folder. Local folder: {}",
                        ddex_message.message_dir_path
                    )
                    .as_str(),
                );

            s3_folder_to_ddex_message_map.insert(s3_path.clone(), ddex_message);
        }
        for folder in message_folders {
            if let Some(ddex_message) = s3_folder_to_ddex_message_map.get(&folder) {
                if ddex_message.excluded {
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
