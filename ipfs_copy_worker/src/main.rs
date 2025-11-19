#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ipfs_copy_worker::run().await?;
    Ok(())
}
