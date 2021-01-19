// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
//consider fuzzy finding  
mod class;
use rand::seq::SliceRandom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching class data...");
    let url = "https://api.devhub.virginia.edu/v1/courses";
    let response: String = reqwest::get(url).await?.text().await?;

    println!("Data successfully recieved. Parsing JSON...");
    let parsed = json::parse(&response)?;

    println!("Printing json...");
    let columns = parsed["class_schedules"]["columns"].members();
    let data = parsed["class_schedules"]["records"].members();

    for header in columns {
        print!("{}\t", header);
    }
    println!();

    // let data = data.filter(|arr| arr[0] == "CS"); // Filtering takes, but we can shadow to keep variable the same.
    let mut class_vec: Vec<class::Class> = Vec::new();
    for row in data {
        let current_class = class::Class::build_from_json(row);
        class_vec.push(current_class);
    }
    let selection: Vec<_> = class_vec.choose_multiple(&mut rand::thread_rng(), 20).collect();
    for course in selection{
        println!("{:?}\n", course);
    }
    println!("Done.");

    return Ok(());
}
