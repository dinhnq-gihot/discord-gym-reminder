use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(Debug, Identifiable, AsChangeset, Queryable, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schedule {
    pub id: i32,
    pub user_id: String,               // Adjust type if necessary
    pub channel_id: String,            // Adjust type if necessary
    pub day: String,                   // Adjust type if necessary
    pub start_time: chrono::NaiveTime, // Use chrono for time
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSchedule<'a> {
    pub user_id: &'a str,                  // Adjust type if necessary
    pub channel_id: &'a str,               // Adjust type if necessary
    pub day: &'a str,                      // Adjust type if necessary
    pub start_time: &'a chrono::NaiveTime, // Use chrono for time
}
