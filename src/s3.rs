use aws_config::meta::region::RegionProviderChain;
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

async fn get_object(client: Client, opt: Opt) -> Result<usize, io::Error> {
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

