use puddleglum::S3;

#[tokio::main]
async fn main() {
    let mut s3object = S3 {
        bucket: "my-bucket".to_string(),
        key: "my-key".to_string(),
        client: Default::default(),
    };

    s3object.build_config_and_client().await;

    
}
