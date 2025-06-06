use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::sqs::SqsEvent;
use aws_sdk_dynamodb::{
    error::SdkError, operation::put_item::PutItemError, types::AttributeValue, Client,
};
use chrono::Utc;
use lambda_runtime::{tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
struct EventBodyObject {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventBodyDetail {
    object: EventBodyObject,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventBody {
    detail: EventBodyDetail,
}

pub(crate) async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let queue_consumer_config = aws_queue_consumer::Config::build();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let dynamo_client = Client::new(&aws_config);

    for event_record in payload.records.iter() {
        let event_body_string = event_record
            .body
            .as_ref()
            .expect("Event's record body is empty!");
        let item = prepare_item(event_body_string, &queue_consumer_config)?;

        let resp = dynamo_client
            .put_item()
            .table_name(queue_consumer_config.message_status_table_name.as_str())
            .set_item(Some(item))
            .condition_expression(format!(
                "attribute_not_exists({})",
                &queue_consumer_config.message_folder_attribute_name
            ))
            .send()
            .await;

        match resp {
            Ok(_) => {
                println!("Message folder for event: {event_body_string} inserted successfully")
            }
            Err(SdkError::ServiceError(err))
                if matches!(err.err(), PutItemError::ConditionalCheckFailedException(_)) =>
            {
                println!("Item already exists, skipping insert.");
            }
            Err(e) => {
                eprintln!("Failed to insert item: {:?}", e);
            }
        }
    }

    Ok(())
}

fn prepare_item(
    event_body_string: &String,
    config: &aws_queue_consumer::Config,
) -> Result<HashMap<String, AttributeValue>, Error> {
    let event_body: EventBody = serde_json::from_str(event_body_string.as_str())?;
    let s3_path = Path::new(&event_body.detail.object.key);
    let message_folder = s3_path
        .parent()
        .expect("Message uploaded to root folder")
        .to_str()
        .expect("Couldn't parse message folder string");

    let mut item: HashMap<String, AttributeValue> = HashMap::new();

    item.insert(
        config.message_folder_attribute_name.to_string(),
        AttributeValue::S(message_folder.to_string()),
    );

    item.insert(
        config.created_timestamp_attribute_name.to_string(),
        AttributeValue::N(Utc::now().timestamp_millis().to_string()),
    );
    item.insert(
        config.updated_timestamp_attribute_name.to_string(),
        AttributeValue::N(Utc::now().timestamp_millis().to_string()),
    );
    item.insert(
        config.processing_status_attribute_name.to_string(),
        AttributeValue::S(config.unprocessed_status_value.to_string()),
    );

    Ok(item)
}
