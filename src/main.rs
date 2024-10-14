mod error;

use aws_config::meta::region::RegionProviderChain;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{Value, json};

use std::{fs::File, io::{self, Write}, path::PathBuf, process::exit};
use aws_sdk_s3::{Client, Config};
use clap::Parser;
use tracing::trace;

#[derive(Debug, Parser)]
struct Opt {
    #[structopt(long)]
    bucket: String,
    #[structopt(long)]
    object: String,
    #[structopt(long)]
    destination: PathBuf,
}

async fn get_object(client: Client, opt: Opt) -> Result<usize, aws_sdk_s3::Error> {
    trace!("bucket:      {}", opt.bucket);
    trace!("object:      {}", opt.object);
    trace!("destination: {}", opt.destination.display());

    let mut file = File::create(opt.destination.clone())?;

    let mut object = client
        .get_object()
        .bucket(opt.bucket)
        .key(opt.object)
        .send()
        .await?;

    let mut byte_count = 0_usize;
    while let Some(bytes) = object.body.try_next().await? {
        let bytes_len = bytes.len();
        file.write_all(&bytes)?;
        trace!("Intermediate write of {bytes_len}");
        byte_count += bytes_len;
    }

    Ok(byte_count)
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    
    match get_object(client, Opt::parse()).await {
        Ok(bytes) => {
            println!("Wrote {bytes}");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
    let response_value = json!({
        "status": "success"
    });

    // 成功時の戻り値を返す
    Ok(response_value)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
