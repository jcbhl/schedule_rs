// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Calling method...");
    run_request().await;
    Ok(())
}
async fn run_request() -> Result<(), reqwest::Error>{
    let url = "https://api.devhub.virginia.edu/v1/courses";
    match reqwest::get(url).await {
        Ok(result) => println!("Responded with : {}", result.status()),
        Err(error) => println!("Error caused by: {}", error),

    }
    //match reqwest::get(url).await {
        //Ok(result) => println!("Responsed with : {}", result.status()),
        //Err(error) => println!("Error caused by: {}", error),
    //}
    //let body = reqwest::get("https://api.devhub.virginia.edu/v1/courses").await?.text().await?;
    // println!("{}",body);
    Ok(())
}
