// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Fetching class data...");
    let url = "https://api.devhub.virginia.edu/v1/courses";
    let response = reqwest::get(url).await?.text().await?;
    println!("{}",response);
    return Ok(());
}
