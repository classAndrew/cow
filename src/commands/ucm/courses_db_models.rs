pub struct Reminder {
    pub user_id: u64,
    pub course_reference_number: i32,
    pub min_trigger: i32,
    pub for_waitlist: bool,
    pub triggered: bool
}

pub struct Trigger {
    pub user_id: u64,
    pub course_reference_number: i32,
    pub min_trigger: i32
}