use super::{musculature::Musculature, schedule::Schedule};
use diesel::prelude::*;

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::schedules_musculatures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Schedule))]
#[diesel(belongs_to(Musculature))]
#[diesel(primary_key(schedule_id, musculature_id))]
pub struct ScheduleMusculature {
    pub schedule_id: i32,
    pub musculature_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schedules_musculatures)]
pub struct NewScheduleMusculature<'a> {
    pub schedule_id: &'a i32,
    pub musculature_id: &'a i32,
}
