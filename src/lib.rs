use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use std::env;
use aws_sdk_s3::primitives::DateTime;

pub struct S3 {
    pub bucket: String,
    pub key: String,
    pub client: Option<Client>,
}

pub struct S3object {
    pub key: Option<String>,
    pub last_modified: Option<DateTime>,
    pub size: Option<i64>,
}

impl S3 {

    async fn check_for_creds(&self) -> Credentials {
        let aws_access_key_id = env::var_os("aws_access_key_id").expect("aws env creds not found").into_string();
        let aws_secret_access_key = env::var_os("aws_secret_access_key").expect("aws env creds not found").into_string();
        let keys = Credentials::new(
            aws_access_key_id.expect("failed aws env creds"),
            aws_secret_access_key.expect("failed aws env creds"),
            None,
            None,
            "dummy",
        );
        return keys;
    }

    pub async fn build_config_and_client(& mut self) {
        let keys = self.check_for_creds().await;
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(keys)
            .load()
            .await;
        let client: Client = Client::new(&config);
        self.client = Some(client);
    }

    pub async fn list_objects(client: &Client, bucket: &str, bucket_contents: Vec<&Object>) ->Vec<&Object> {
        let mut response = client
            .list_objects_v2()
            .bucket(bucket.to_owned())
            .max_keys(10)
            .into_paginator()
            .send();
    
        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        bucket_contents.push(object);
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}")
                }
            }
        }
    
        bucket_contents
    }
    
    


}