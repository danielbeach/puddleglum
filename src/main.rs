use puddleglum::S3;

#[tokio::main]
async fn main() {
    let mut s3object = S3 {
        bucket: "rules-assist".to_string(),
        prefix: "non_fis_rules/high-fraud-merchant/".to_string(),
        client: Default::default(),
    };

    s3object.build_config_and_client().await;

    // get all the objects in the bucket and prefiex
    let em = s3object.list_objects().await;
    //println!("{:?}", em);

    // get the most recent object in the bucket and prefix
    let most_recent = s3object.get_most_recent_file().await;
    println!("{:?}", most_recent);

    // how many files in the last 24 hours
    let files_last_24_hours = s3object.how_many_files_last_24_hours().await;
    println!("files in the last 24 hours: {:?}", files_last_24_hours);

    // how many files in the last 48 hours
    let files_last_48_hours = s3object.how_many_files_last_48_hours().await;
    println!("files in the last 48 hours: {:?}", files_last_48_hours);
}
