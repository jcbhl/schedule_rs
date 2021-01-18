use json;
#[derive(Debug)]
struct Class{
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
impl Class{
    //Take json value by reference and instantiate fields
    pub fn build_from_json(val: &JsonValue) -> Self{
        Self{
            subject: val[0],
            catalog_number: val[1],
            class_section: val[2],
            class_number: val[3],
            class_
        }
    }
}

#[derive(Debug)]
struct Schedule{
    meeting_days: String
    meeting_time_start: String,
    meeting_time_end: String,
}
impl Schedule{
    pub fn new(meeting_days: String, meeting_time_start: String, meeting_time_end: String) -> Self {
        Self{
            meeting_days,
            meeting_time_start,
            meeting_time_end,    
        }
    }
}