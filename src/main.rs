// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
use json;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Fetching class data...");
    let url = "https://api.devhub.virginia.edu/v1/courses";
    let response: String = reqwest::get(url).await?.text().await?;

    println!("Data successfully recieved. Parsing JSON");
    let parsed = json::parse(&response).unwrap();
    // println!("{}",parsed["class_schedules"]["records"]);
    println!("Printing json...");
    let columns = parsed["class_schedules"]["columns"].members();
    let data = parsed["class_schedules"]["records"].members();

    // for (key, value) in parsed["class_schedules"].entries().take(10){
    //     println!("Key: {}", key);
    //     println!("Value: {}", value);
    // }
    for header in columns{
        print!("{}\t", header);
    }
    
    let data = data.filter(|arr| arr[0] == "CS"); // Filtering takes, but we can shadow to keep variable the same.
    for row in data{
        println!("{}", row);
    }
    println!("Done.");


    return Ok(());
}
