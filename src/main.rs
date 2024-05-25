use puddleglum::S3;

#[tokio::main]
async fn main() {
    let mut s3object = S3 {
        bucket: "some-bucket".to_string(),
        prefix: "some-prefix".to_string(),
        client: Default::default(),
    };

    s3object.build_config_and_client().await;

    // get all the objects in the bucket and prefiex
    let em = s3object.list_objects().await;
    println!("{:?}", em);

    // get the most recent object in the bucket and prefix
    let most_recent = s3object.get_most_recent_file().await;
    println!("{:?}", most_recent);
}
