use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{Error, LambdaEvent};
use owen::contracts::{ContractsConfig, ContractsManager};

pub(crate) async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    // let contracts_config=ContractsConfig{
    //  rpc_url: String,
    //  private_key: String,
    //  ddex_sequencer_address: Address,
    //  use_kms: false,
    //  signer_kms_id: None,
    // }

    // let contracts_manager = ContractsManager::build(config)
    println!("all good");
    println!("event: {event:?}");
    Ok(())
}
