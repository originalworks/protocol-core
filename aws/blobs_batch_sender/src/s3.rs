use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use blobs_batch_sender::BlobsBatchSenderConfig;
use lambda_runtime::Error;
use owen::blobs_queue::BlobsQueueS3JsonFile;
use tokio::io::AsyncReadExt;

pub struct BlobsStorage {
    client: Client,
    bucket_name: String,
}

impl BlobsStorage {
    pub async fn build(config: &BlobsBatchSenderConfig) -> Result<Self, Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let aws_config = aws_config::from_env().region(region_provider).load().await;
        let client: Client = Client::new(&aws_config);

        Ok(Self {
            client,
            bucket_name: config.blobs_temp_storage_bucket_name.clone(),
        })
    }

    pub async fn read(&self, blobhashes: Vec<String>) -> Result<Vec<BlobsQueueS3JsonFile>, Error> {
        let mut blobs: Vec<BlobsQueueS3JsonFile> = Vec::new();

        for blobhash in blobhashes {
            let key = format!("blobs/{}.json", blobhash);
            let resp = self
                .client
                .get_object()
                .bucket(self.bucket_name.clone())
                .key(key)
                .send()
                .await?;

            let mut body = resp.body.into_async_read();
            let mut contents = String::new();
            body.read_to_string(&mut contents).await?;
            let blob_transaction_data: BlobsQueueS3JsonFile =
                serde_json::from_str(&contents).unwrap();
            blobs.push(blob_transaction_data);
        }

        Ok(blobs)
    }
}
