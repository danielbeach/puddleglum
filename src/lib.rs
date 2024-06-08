use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use std::env;
use aws_sdk_s3::primitives::DateTime;
use aws_smithy_types_convert::date_time::DateTimeExt;



pub struct S3 {
    pub bucket: String,
    pub prefix: String,
    pub results: Option<Vec<S3object>>,
    pub date_sorted: Option<Vec<S3object>>,
}

#[derive(Debug, Clone)]
pub struct S3object {
    pub key: Option<String>,
    pub last_modified: Option<DateTime>,
    pub size: Option<i64>,
}


impl S3 {

    pub async fn get_em(&mut self) {
        let client = build_config_and_client().await;
        let mut response = client
            .list_objects_v2()
            .prefix(&self.prefix)
            .bucket(&self.bucket.to_owned())
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
    
        self.results = Some(all_results);
    }

    pub async fn count_number_of_files(&self) -> usize {
        let count = self.results.clone().unwrap().len();
        return count;
    }

    pub fn quick_sort_s3_objects_by_date(&mut self, mut objects: Vec<S3object>) -> Vec<S3object> {
        if objects.len() <= 1 {
            return objects;
        }
        let pivot = objects.remove(0);
        let mut less_than_pivot = Vec::new();
        let mut greater_than_pivot = Vec::new();
        for object in objects {
            if object.last_modified.unwrap() <= pivot.last_modified.unwrap() {
                less_than_pivot.push(object);
            } else {
                greater_than_pivot.push(object);
            }
        }
        let mut sorted = self.quick_sort_s3_objects_by_date(less_than_pivot);
        sorted.push(pivot);
        sorted.append(&mut self.quick_sort_s3_objects_by_date(greater_than_pivot));
        self.date_sorted = Some(sorted.clone());
        return sorted;
    }

    pub async fn get_most_recent_file(&mut self) -> S3object {
        let mut objects = self.results.clone().unwrap();
        if self.date_sorted.is_none() {
            objects = self.quick_sort_s3_objects_by_date(objects);
        }
        return objects.pop().unwrap();
    }

    pub async fn get_n_most_recent_files(&mut self, n: usize) -> Vec<S3object> {
        let mut objects = self.results.clone().unwrap();
        if self.date_sorted.is_none() { 
            objects = self.quick_sort_s3_objects_by_date(objects);
        }
        let mut most_recent_files: Vec<S3object> = Vec::new();
        for _ in 0..n {
            most_recent_files.push(objects.pop().unwrap());
        }
        return most_recent_files;
    }

    pub async fn get_n_days_ago_files(&mut self, n: i64) -> Vec<S3object> {
        let mut objects = self.results.clone().unwrap();
        if self.date_sorted.is_none() {
            objects = self.quick_sort_s3_objects_by_date(objects);
        }
        let mut n_days_ago_files: Vec<S3object> = Vec::new();
        let n_days_ago = chrono::Utc::now() - chrono::Duration::days(n);
        for object in objects {
            if let Ok(last_modified) = object.last_modified.unwrap().to_chrono_utc() {
                if last_modified >= n_days_ago {
                    n_days_ago_files.push(object);
                }
            }
        }
        return n_days_ago_files;
    }

    pub async fn get_n_weeks_ago_files(&mut self, n: i64) -> Vec<S3object> {
        let mut objects = self.results.clone().unwrap();
        if self.date_sorted.is_none() {
            objects = self.quick_sort_s3_objects_by_date(objects);
        }
        let mut n_weeks_ago_files: Vec<S3object> = Vec::new();
        let n_weeks_ago = chrono::Utc::now() - chrono::Duration::weeks(n);
        for object in objects {
            if let Ok(last_modified) = object.last_modified.unwrap().to_chrono_utc() {
                if last_modified >= n_weeks_ago {
                    n_weeks_ago_files.push(object);
                }
            }
        }
        return n_weeks_ago_files;
    }

    pub async fn get_size_of_files_in_gb(&self) -> f64 {
        let mut total_size = 0;
        for object in self.results.clone().unwrap() {
            total_size += object.size.unwrap();
        }
        let total_size_gb = total_size as f64 / 1_000_000_000.0;
        return total_size_gb;
    }

    pub async fn get_largest_file(&self) -> S3object {
        let mut largest_file = S3object { key: None, last_modified: None, size: None };
        for object in self.results.clone().unwrap() {
            if object.size.unwrap() > largest_file.size.unwrap_or(0) {
                largest_file = object;
            }
        }
        return largest_file;
    }

}

async fn check_for_creds() -> Credentials {
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

async fn build_config_and_client() -> Client {
    let keys = check_for_creds().await;
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(keys)
        .load()
        .await;
    let client: Client = Client::new(&config);
    return client;
}
