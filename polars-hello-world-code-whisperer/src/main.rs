/*
Polars hello world script that uses AWS Code Catalyst and Code Whisperer
*/
use polars::prelude::*;
use aws_types::credentials::SharedCredentialsProvider;
use std::error::Error;
use tokio;
use aws_sdk_s3::model::PutObjectRequest;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use aws_sdk_s3::{Client, Config, Region};
use aws_types::credentials::SharedCredentialsProvider;
use std::error::Error;


pub fn calculate() -> Result<DataFrame, PolarsError> {
    // Read the CSV data using CsvReader
    let file = std::fs::File::open("data/iris.csv").unwrap();

    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}


async fn upload_object(s3_client: &Client, bucket: &str, key: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    s3_client.put_object(PutObjectRequest::builder()
        .bucket(bucket)
        .key(key)
        .body(buffer.into())
        .build())
        .send()
        .await?;

    Ok(())
}


#[tokio::main]
async fn altdata_s3() -> Result<(), Box<dyn Error>> {
    // Load the AWS credentials from the environment
    let shared_config = aws_config::from_env().load().await;
    
    // Create an S3 client
    let s3_client = Client::new(&shared_config);
    
    // List buckets
    let buckets = s3_client.list_buckets().send().await?;
    println!("Buckets:");
    for bucket in buckets.buckets().unwrap_or_default() {
        println!("  {}", bucket.name().unwrap_or_default());
    }
    
    Ok(())
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}