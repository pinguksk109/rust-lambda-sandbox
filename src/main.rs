mod error;
mod s3;

use serde_json::Value;
use std::{collections::HashMap, path};

use lambda_runtime::{service_fn, Error, LambdaEvent};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let contents = s3::get_object(&"sample_word_document.docx".to_string()).await?;

    let json_contents = match serde_json::from_slice(&contents) {
        Ok(val) => val,
        Err(_) => serde_json::json!({"error": "Invalid Json"}),
    };

    Ok(json_contents)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
