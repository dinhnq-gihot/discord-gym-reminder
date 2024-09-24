// src/models.rs
use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(
    Debug, Identifiable, AsChangeset, Queryable, Selectable, Associations, PartialEq, Clone,
)]
#[diesel(table_name = crate::schema::exercises)]
#[diesel(belongs_to(Musculature, foreign_key = musculature_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub impact: String,
    pub level: String,
    pub description: String,
    pub video: Vec<Option<String>>,
    pub male_weight: String,
    pub female_weight: String,
    pub musculature_id: i32,
}

#[derive(Debug, Identifiable, AsChangeset, Queryable, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::musculature)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Musculature {
    pub id: i32,
    pub name: String,
    pub note: Option<String>, // Adjusted for Nullable<Text>
}

#[derive(Debug, Identifiable, AsChangeset, Queryable, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schedule {
    pub id: i32,
    pub user_id: String,                   // Adjust type if necessary
    pub day: String,                       // Adjust type if necessary
    pub start_time: chrono::NaiveTime,     // Use chrono for time
    pub musculatures: Vec<Option<String>>, // Adjust type if necessary
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSchedule<'a> {
    pub user_id: &'a str,                  // Adjust type if necessary
    pub day: &'a str,                      // Adjust type if necessary
    pub start_time: &'a chrono::NaiveTime, // Use chrono for time
    pub musculatures: &'a Vec<String>,     // Adjust type if necessary
}
