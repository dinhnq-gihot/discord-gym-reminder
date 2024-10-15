use diesel::prelude::*;

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub level: i32,
    pub point: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
}
