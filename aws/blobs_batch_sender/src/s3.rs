use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use lambda_runtime::Error;
use owen::blob::BlobTransactionData;
use tokio::io::AsyncReadExt;

pub async fn read_blobs(blobhashes: Vec<String>) -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let bucket = env::var("BLOBS_TEMP_STORAGE_BUCKET_NAME")
        .expect(format!("Missing env variable: BLOBS_TEMP_STORAGE_BUCKET_NAME").as_str());

    let mut blobs: Vec<BlobTransactionData> = Vec::new();

    for blobhash in blobhashes {
        let key = format!("blobs/{}.json", blobhash);
        let resp = client.get_object().bucket(&bucket).key(key).send().await?;

        let mut body = resp.body.into_async_read();
        let mut contents = String::new();
        body.read_to_string(&mut contents).await?;
        let blob_transaction_data: BlobTransactionData = serde_json::from_str(&contents).unwrap();
        blobs.push(blob_transaction_data);
    }

    let commitments: Vec<Vec<u8>> = blobs
        .iter()
        .map(|blob| blob.kzg_commitment.clone())
        .collect();
    println!("{commitments:?}");

    Ok(())
}
