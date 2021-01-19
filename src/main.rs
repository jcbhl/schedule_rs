// ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
//consider fuzzy finding  
use structopt::StructOpt;

mod class;

#[derive(StructOpt)]
struct Command{
    desired_class_title: String,
    // class_type: ClassTypes,
}
async fn request_and_parse() -> Result<json::JsonValue, Box<dyn std::error::Error>> {
    //Todo actually handle error if we cannot make a request to the API 
    let url = "https://api.devhub.virginia.edu/v1/courses";
    let response: String = reqwest::get(url).await?.text().await?;
    let parsed = json::parse(&response)?;
    Ok(parsed)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo cache these results or allow for longer-running program - api call time sucks UX wise
    let parsed = request_and_parse().await?;
    let _column_labels = parsed["class_schedules"]["columns"].members();
    let data = parsed["class_schedules"]["records"].members();

    let mut class_vec: Vec<class::Class> = Vec::new();
    for row in data {
        let current_class = class::Class::build_from_json(row);
        class_vec.push(current_class);
    }
    let args = Command::from_args();
    println!("Searching for all classes matching {}", &args.desired_class_title);
    let mut matching_vec = Vec::new();
    for class in class_vec{
        if class.class_title.contains(&args.desired_class_title){
            matching_vec.push(class);
        }
    }
    println!("Found {} classes that match... Printing", matching_vec.len());
    for matched_class in matching_vec{
        println!("{:?}", matched_class);
    }

    Ok(())
}
