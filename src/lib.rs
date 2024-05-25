use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use std::env;
use std::future::IntoFuture;
use aws_sdk_s3::primitives::DateTime;

pub struct S3 {
    pub bucket: String,
    pub prefix: String,
    pub client: Option<Client>,
}

#[derive(Debug)]
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

    pub async fn get_em(client: &Client, bucket: &str, prefix: &str) -> Vec<S3object> {
        let mut response = client
            .list_objects_v2()
            .prefix(prefix)
            .bucket(bucket.to_owned())
            .max_keys(50)
            .into_paginator()
            .send();
        let mut all_results: Vec<S3object> = Vec::new();

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        let s3object = S3object {
                            key: object.key().map(String::from),
                            last_modified: object.last_modified().cloned(),
                            size: object.size(),
                        };
                        all_results.push(s3object);
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}")
                }
            }
        }
    
        return all_results;
    }

    pub async fn list_objects(&self) -> Vec<S3object> {
        let client = self.client.as_ref().expect("client not found");
        let bucket = &self.bucket;
        let prefix = &self.prefix;
        return S3::get_em(client, bucket, prefix).await;
    }

    pub async fn get_most_recent_file(&self) -> S3object {
        let objects = self.list_objects();
        let mut most_recent = S3object {
            key: None,
            last_modified: None,
            size: None,
        };
        for object in objects.await {
            if most_recent.last_modified.is_none() {
                most_recent = object;
            } else {
                if object.last_modified.unwrap() > most_recent.last_modified.unwrap() {
                    most_recent = object;
                }
            }
        }
        return most_recent;
    }

    pub async fn is_most_recent_file_empty(&self) -> bool {
        let most_recent = self.get_most_recent_file();
        let key = most_recent.await.size;
        if key.is_none() {
            return true;
        } else {
            return false;
        }
    }
}