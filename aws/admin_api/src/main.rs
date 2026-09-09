use admin_api::Config;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use lambda_http::{
    http::StatusCode, run, service_fn, tracing, Body, Error, Request, RequestExt, Response,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageStatus {
    id: String,
    message_folder: String,
    processing_status: Option<String>,
    created_timestamp: Option<i64>,
    updated_timestamp: Option<i64>,
    owen_instance: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let config = Config::build();
    let method = event.method().as_str();
    let path = event.raw_http_path();
    let path_str = path.as_ref();

    // OPTIONS / CORS are handled by API Gateway HttpApi CorsConfiguration.
    if method == "OPTIONS" {
        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::Empty)
            .expect("failed to build response"));
    }

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let dynamo = Client::new(&aws_config);

    let result = match (method, path_str) {
        ("GET", "/messages") | ("GET", "/messages/") => {
            list_messages(&event, &dynamo, &config).await
        }
        ("GET", path) if path.starts_with("/messages/") => {
            let folder = &path["/messages/".len()..];
            if folder.is_empty() {
                Err(ApiError::bad_request("messageFolder is required"))
            } else {
                match urlencoding::decode(folder) {
                    Ok(decoded) => get_message(decoded.as_ref(), &dynamo, &config).await,
                    Err(_) => Err(ApiError::bad_request("Invalid messageFolder encoding")),
                }
            }
        }
        _ => Err(ApiError::not_found("Route not found")),
    };

    match result {
        Ok(response) => Ok(response),
        Err(err) => Ok(json_response(
            err.status,
            json!({ "error": err.message }).to_string(),
        )),
    }
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

async fn list_messages(
    event: &Request,
    dynamo: &Client,
    config: &Config,
) -> Result<Response<Body>, ApiError> {
    let params = event.query_string_parameters();
    let status = params
        .first("status")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiError::bad_request("Query parameter 'status' is required"))?;

    let limit = params
        .first("limit")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(25)
        .clamp(1, 100);

    let mut query = dynamo
        .query()
        .table_name(&config.message_status_table_name)
        .index_name(&config.processing_status_index_name)
        .key_condition_expression("#status = :status")
        .expression_attribute_names("#status", &config.processing_status_attribute_name)
        .expression_attribute_values(":status", AttributeValue::S(status.to_string()))
        .limit(limit)
        .scan_index_forward(false);

    if let Some(cursor) = params.first("cursor").filter(|c| !c.is_empty()) {
        let start_key = decode_cursor(cursor)?;
        query = query.set_exclusive_start_key(Some(start_key));
    }

    let result = query
        .send()
        .await
        .map_err(|e| ApiError::internal(format!("DynamoDB query failed: {e}")))?;

    let items: Vec<MessageStatus> = result
        .items
        .unwrap_or_default()
        .into_iter()
        .filter_map(|item| item_to_message(item, config))
        .collect();

    let next_cursor = result
        .last_evaluated_key
        .map(|key| encode_cursor(&key))
        .transpose()?;

    let body = json!({
        "data": items,
        "nextCursor": next_cursor,
    });

    Ok(json_response(StatusCode::OK, body.to_string()))
}

async fn get_message(
    message_folder: &str,
    dynamo: &Client,
    config: &Config,
) -> Result<Response<Body>, ApiError> {
    let result = dynamo
        .get_item()
        .table_name(&config.message_status_table_name)
        .key(
            &config.message_folder_attribute_name,
            AttributeValue::S(message_folder.to_string()),
        )
        .send()
        .await
        .map_err(|e| ApiError::internal(format!("DynamoDB get_item failed: {e}")))?;

    let item = result
        .item
        .ok_or_else(|| ApiError::not_found(format!("Message not found: {message_folder}")))?;

    let message = item_to_message(item, config)
        .ok_or_else(|| ApiError::internal("Failed to parse DynamoDB item"))?;

    Ok(json_response(
        StatusCode::OK,
        serde_json::to_string(&message).unwrap(),
    ))
}

fn item_to_message(item: HashMap<String, AttributeValue>, config: &Config) -> Option<MessageStatus> {
    let message_folder = attr_s(&item, &config.message_folder_attribute_name)?;
    Some(MessageStatus {
        id: message_folder.clone(),
        message_folder,
        processing_status: attr_s(&item, &config.processing_status_attribute_name),
        created_timestamp: attr_n(&item, &config.created_timestamp_attribute_name),
        updated_timestamp: attr_n(&item, &config.updated_timestamp_attribute_name),
        owen_instance: attr_s(&item, &config.owen_instance_attribute_name),
    })
}

fn attr_s(item: &HashMap<String, AttributeValue>, key: &str) -> Option<String> {
    item.get(key).and_then(|v| v.as_s().ok().cloned())
}

fn attr_n(item: &HashMap<String, AttributeValue>, key: &str) -> Option<i64> {
    item.get(key)
        .and_then(|v| v.as_n().ok())
        .and_then(|n| n.parse().ok())
}

fn encode_cursor(key: &HashMap<String, AttributeValue>) -> Result<String, ApiError> {
    let mut map = serde_json::Map::new();
    for (k, v) in key {
        map.insert(k.clone(), attribute_to_json(v)?);
    }
    Ok(URL_SAFE_NO_PAD.encode(Value::Object(map).to_string().as_bytes()))
}

fn decode_cursor(cursor: &str) -> Result<HashMap<String, AttributeValue>, ApiError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| ApiError::bad_request("Invalid cursor"))?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|_| ApiError::bad_request("Invalid cursor JSON"))?;
    let obj = value
        .as_object()
        .ok_or_else(|| ApiError::bad_request("Invalid cursor object"))?;

    let mut key = HashMap::new();
    for (k, v) in obj {
        key.insert(k.clone(), json_to_attribute(v)?);
    }
    Ok(key)
}

fn attribute_to_json(attr: &AttributeValue) -> Result<Value, ApiError> {
    if let Ok(s) = attr.as_s() {
        return Ok(json!({ "S": s }));
    }
    if let Ok(n) = attr.as_n() {
        return Ok(json!({ "N": n }));
    }
    Err(ApiError::internal("Unsupported cursor attribute type"))
}

fn json_to_attribute(value: &Value) -> Result<AttributeValue, ApiError> {
    if let Some(s) = value.get("S").and_then(|v| v.as_str()) {
        return Ok(AttributeValue::S(s.to_string()));
    }
    if let Some(n) = value.get("N").and_then(|v| v.as_str()) {
        return Ok(AttributeValue::N(n.to_string()));
    }
    Err(ApiError::bad_request("Unsupported cursor attribute type"))
}

fn json_response(status: StatusCode, body: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .expect("failed to build response")
}
