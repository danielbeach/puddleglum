use puddleglum;


#[tokio::main]
async fn main()
{
    let bucket = "rules-assist".to_string();
    let prefix = "wall-rules-v1.3/2024/06".to_string();
    
    let mut s3 = puddleglum::S3 { bucket: bucket.clone(), prefix: prefix.clone(), results: None, date_sorted: None};
    s3.get_em().await;

    // Count number of files
    //let count = s3.count_number_of_files().await;
    //println!("Number of files: {}", count);

    // Get the most recent file
    //let most_recent_file = s3.get_most_recent_file().await;
    //println!("Most recent file: {:?}", most_recent_file);

    // Get the 5 most recent files
    //let most_recent_files = s3.get_n_most_recent_files(5).await;
    //println!("5 most recent files: {:?}", most_recent_files);

    //let days_ago_files = s3.get_n_days_ago_files(2).await;
    //println!("Files from 2 days ago: {:?}", days_ago_files);

    //let weeks_age_files = s3.get_n_weeks_ago_files(2).await;
    //println!("Files from 2 weeks ago: {:?}", weeks_age_files);

    //let size_of_files = s3.get_size_of_files_in_gb().await;
    //println!("Size of files in GB: {}", size_of_files);

    //let largest_file = s3.get_largest_file().await;
    //println!("Largest file: {:?}", largest_file);

    //let todays_files = s3.get_todays_files().await;
    //println!("Todays files: {:?}", todays_files);

    //let grouped_by_day = s3.count_files_and_group_by_day_for_n_days(2).await;
    //println!("Grouped by day: {:?}", grouped_by_day);

    //let grouped_by_week = s3.count_files_by_last_n_weeks(2).await;
    //println!("Grouped by week: {:?}", grouped_by_week);

}

