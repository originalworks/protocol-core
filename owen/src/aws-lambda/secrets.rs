use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct OwenSecretEnvs {
    RPC_URL: String,
    PRIVATE_KEY: String,
}

pub async fn set_secret_envs(aws_main_config: &aws_config::SdkConfig) -> Result<()> {
    let client = aws_sdk_secretsmanager::Client::new(&aws_main_config);
    let owen_lambda_secrets_name = env::var("OWEN_LAMBDA_SECRETS_NAME")
        .expect(format!("Missing env variable: OWEN_LAMBDA_SECRETS_NAME").as_str());

    let response = client
        .get_secret_value()
        .secret_id(owen_lambda_secrets_name)
        .send()
        .await?;

    let secrets_json_string = response
        .secret_string()
        .expect("Could not retrieve secret string from AWS SM");

    let owen_secret_envs: OwenSecretEnvs = serde_json::from_str(secrets_json_string)?;

    env::set_var("RPC_URL", owen_secret_envs.RPC_URL);
    env::set_var("PRIVATE_KEY", owen_secret_envs.PRIVATE_KEY);

    Ok(())
}
