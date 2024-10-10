// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_s3::{Client, Error as S3Error, config::{Builder, Region}};
// use serde_json::Value;
// use std::{io::Write};
// use futures::stream::StreamExt;

// pub async fn download_file_from_s3(path: String) -> Result<String, S3Error> {
//     let region_provider = RegionProviderChain::first_try(Some(Region::new("us-west-2"))).or_default_provider();
//     let shared_config = aws_config::from_env().region(region_provider).load().await;
//     let config = Config::builder()
//         .region(region_provider.region().await) // 既存のリージョンを使用
//         .endpoint_url("http://localhost:55510/") // MinIOのエンドポイントを設定
//         .credentials_provider(shared_config.credentials_provider) // 認証情報プロバイダー
//         .http_client(shared_config.http_client) // 既存のHTTPクライアントを使用
//         .build();

//     let client = Client::from_conf(config);

//     let mut object_output = client
//         .get_object()
//         .bucket("test-bucket")
//         .key(path.clone())
//         .send()
//         .await?;

//     let mut content = Vec::new();

//     while let Some(bytes) = object_output.body.try_next().await? {
//         content.extend(bytes);
//     }

//     String::from_utf8(content).map_err(|e| S3Error::from(e))
// }