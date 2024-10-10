use aws_sdk_s3::{Client, Config, Error};
use aws_config::meta::region::RegionProviderChain;
use bytes::Bytes;

const BUCKET_NAME: &str = "your-bucket-name";
const REGION: &str = "us-east-1";

pub async fn download_file_from_s3(path: &str) -> Result<Bytes, Error> {
    let region_provider = RegionProviderChain::first_try(REGION).or_else(REGION);
    
    let config = aws_config::from_env().region(region_provider).load().await;
    
    let client = Client::new(&config);

    let resp = client.get_object()
        .bucket(BUCKET_NAME)
        .key(path)
        .send()
        .await?;

    let mut contents = Bytes::new();
    if let Some(body) = resp.body {
        contents = body.collect().await?.into_bytes(); // AggregatedBytesをBytesに変換
    }

    Ok(contents)
}
