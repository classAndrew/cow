use bitflags::bitflags;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

pub struct Class {
    pub id: i32,
    pub term: i32,
    pub course_reference_number: i32,
    pub course_number: String,
    pub campus_description: Option<String>,
    pub course_title: Option<String>,
    pub credit_hours: u8,
    pub maximum_enrollment: i16,
    pub enrollment: i16,
    pub seats_available: i16,
    pub wait_capacity: i16,
    pub wait_available: i16
}

bitflags! {
    pub struct Days: u8 {
        const BASE = 0;
        const SUNDAY = 1;
        const MONDAY = 2;
        const TUESDAY = 4;
        const WEDNESDAY = 8;
        const THURSDAY = 16;
        const FRIDAY = 32;
        const SATURDAY = 64;
    }
}

#[repr(u8)]
#[derive(FromPrimitive)]
pub enum MeetingType {
    Lecture = 1,
    Discussion = 2,
    Lab = 3,
    Fieldwork = 4,
    Seminar = 5,
    IndividualStudy = 6,
    Tutorial = 7,
    Studio = 8,
    Practicum = 9,
    Exam = 10,
    Project = 11,
    Internship = 12
}

impl TryFrom<u8> for MeetingType {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(v).ok_or(())
    }
}

pub struct Meeting {
    pub class_id: i32,
    pub begin_time: String,
    pub end_time: String,
    pub begin_date: String,
    pub end_date: String,
    pub building: Option<String>,
    pub building_description: Option<String>,
    pub campus: Option<String>,
    pub campus_description: Option<String>,
    pub room: Option<String>,
    pub credit_hour_session: f32,
    pub hours_per_week: f32,
    pub in_session: Days,
    pub meeting_type: MeetingType
}

pub struct Professor {
    pub id: i32,
    pub rmp_id: Option<i32>,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub email: Option<String>,
    pub department: Option<String>,
    pub num_ratings: i32,
    pub rating: f32
}