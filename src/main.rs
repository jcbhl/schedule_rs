// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    run_request();
    Ok(())
}
async fn run_request() -> Result<(), reqwest::Error>{
    let _url = "https://api.devhub.virginia.edu/v1/courses";
    //match reqwest::get(url).await {
        //Ok(result) => println!("Responsed with : {}", result.status()),
        //Err(error) => println!("Error caused by: {}", error),
    //}
    let response = reqwest::get("https://api.devhub.virginia.edu/v1/courses").await?.text().await?;
    Ok(())
}
