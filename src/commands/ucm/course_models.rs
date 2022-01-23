use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Course {
    // there's no guarantee which one can be null
    pub id: u64,
    pub term_effective: Option<String>,
    pub course_number: Option<String>,
    pub subject: Option<String>,
    pub subject_code: Option<String>,
    pub college: Option<String>,
    pub college_code: Option<String>,
    pub department: Option<String>,
    pub department_code: Option<String>,
    pub course_title: Option<String>,
    pub credit_hour_indicator: Option<String>,
    pub subject_description: Option<String>,
    pub course_description: Option<String>,
    pub division: Option<String>,
    pub term_start: Option<String>,
    pub term_end: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CourseSearchConfig {
    pub config: Option<String>,
    pub display: Option<String>,
    pub title: Option<String>,
    pub required: bool,
    pub width: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DisplaySettings {
    pub enrollment_display: Option<String>,
    pub waitlist_display: Option<String>,
    pub cross_list_display: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CourseList {
    pub success: bool,
    pub total_count: u64,
    pub data: Vec<Course>,
    pub page_offset: u64,
    pub page_max_size: u64,
    pub path_mode: Option<String>,
    pub course_search_results_configs: Vec<CourseSearchConfig>,
    pub display_settings: DisplaySettings,
    pub is_plan_by_crn_set_for_term: bool
}
