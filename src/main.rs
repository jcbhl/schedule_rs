// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
fn main(){
    println!("Loading class data");
    run();
}
async fn run(){
    match get_class_data().await{
        Ok(result) => println!(),
        Err(error) => println!("error"),
    }
}
async fn get_class_data() -> Result<String,reqwest::Error>{
    let url = "https://api.devhub.virginia.edu/v1/courses";
    return reqwest::get(url).await?.text().await;
}
