extern crate chrono;

use super::schema::excel2pg;

#[derive(Debug, Insertable)]
#[table_name="excel2pg"]
pub struct Excel2pg {
    pub file: String,
    pub sheet: String,
    pub fdate: chrono::NaiveDateTime,
    pub idate: chrono::NaiveDateTime,
    pub s001: String,
    pub s002: String,
    pub s003: String
}
