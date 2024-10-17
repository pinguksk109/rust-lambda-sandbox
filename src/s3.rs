use crate::error::S3Error;

use aws_config::{
    default_provider::credentials, meta::region::RegionProviderChain,
    profile::ProfileFileCredentialsProvider,
};
use aws_sdk_s3::{config::Region, Client};

pub async fn get_object(path: &String) -> Result<Vec<u8>, S3Error> {
    let bucket = "test-bucket";
    let region = "ap-northeast-1";

    let region_provider =
        RegionProviderChain::first_try(Region::new(region.clone())).or_else("ap-northeast-1");

    // ローカルのminioに対して実行するときに必要
    let credentials = ProfileFileCredentialsProvider::builder()
        .profile_name("default")
        .build();
    let config = aws_config::from_env()
        .endpoint_url("http://localhost:55510")
        .credentials_provider(credentials)
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);

    let mut object = client.get_object().bucket(bucket).key(path).send().await?;

    let mut byte_data = Vec::new();
    while let Some(bytes) =
        object.body.try_next().await.map_err(|err| {
            S3Error::new(format!("Failed to read from S3 download stream: {err:?}"))
        })?
    {
        byte_data.extend_from_slice(&bytes);
    }

    Ok(byte_data)
}
