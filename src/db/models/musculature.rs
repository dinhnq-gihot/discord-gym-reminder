use diesel::prelude::*;

#[derive(Debug, Identifiable, AsChangeset, Queryable, Selectable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::musculatures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Musculature {
    pub id: i32,
    pub name: String,
    pub note: Option<String>, // Adjusted for Nullable<Text>
}