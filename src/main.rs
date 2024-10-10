mod s3;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{Value, json};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let contents = s3::download_file_from_s3("sample_word_document.docx");
    
    let response = json!({
        "statusCode": 200,
        "isBase64Encoded": true,
        "body": ""
    });

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
