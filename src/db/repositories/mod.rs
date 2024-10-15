pub mod exercise;
pub mod musculature;
pub mod schedule;
pub mod user;

use diesel::result::Error;

pub type DbResult<T> = Result<T, Error>;
