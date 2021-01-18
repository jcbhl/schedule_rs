#[derive(Debug)]
pub struct Class {
    subject: String,
    catalog_number: u32,
    class_section: u32,
    class_number: u32,
    class_title: String,
    class_topic_formal_desc: String,
    instructor: String,
    enrollment_capacity: u32,
    schedule: Schedule,
    term: String,
    term_desc: String,
}
impl Class {
    //Take json value by reference and instantiate fields
    pub fn build_from_json(val: &json::JsonValue) -> Self {
        Self {
            subject: String::from(val[0].as_str().unwrap()),
            catalog_number: val[1].as_str().unwrap().parse::<u32>().unwrap(),
            class_section: val[2].as_str().unwrap().parse::<u32>().unwrap(),
            class_number: val[3].as_u32().unwrap(),
            class_title: String::from(val[4].as_str().unwrap()),
            class_topic_formal_desc: String::from(val[5].as_str().unwrap()),
            instructor: String::from(val[6].as_str().unwrap()),
            enrollment_capacity: val[7].as_u32().unwrap(),
            schedule: Schedule::new(
                val[8].as_str().unwrap(),
                val[9].as_str().unwrap(),
                val[10].as_str().unwrap(),
            ),
            term: String::from(val[11].as_str().unwrap()),
            term_desc: String::from(val[12].as_str().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Schedule {
    meeting_days: String,
    meeting_time_start: String,
    meeting_time_end: String,
}
impl Schedule {
    pub fn new(meeting_days: &str, meeting_time_start: &str, meeting_time_end: &str) -> Self {
        Self {
            meeting_days: String::from(meeting_days),
            meeting_time_start: String::from(meeting_time_start),
            meeting_time_end: String::from(meeting_time_end),
        }
    }
}
