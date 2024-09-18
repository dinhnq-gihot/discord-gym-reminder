// src/models.rs
use crate::schema::{exercises, musculature};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(
    Debug, Identifiable, AsChangeset, Queryable, Selectable, Associations, PartialEq, Clone,
)]
#[diesel(table_name = crate::schema::exercises)]
#[diesel(belongs_to(Musculature, foreign_key = musculature_id))]
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
pub struct Musculature {
    pub id: i32,
    pub name: String,
    pub note: Option<String>, // Adjusted for Nullable<Text>
}
