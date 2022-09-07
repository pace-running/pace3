use crate::schema::runners;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = runners)]
pub struct NewRunner<'a> {
    pub start_number: i32,
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
    pub team: Option<&'a str>,
    pub email: Option<&'a str>,
    pub starting_point: &'a str,
    pub running_level: &'a str,
    pub donation: &'a str,
}

#[derive(Queryable)]
pub struct Runner {
    pub id: i32,
    pub start_number: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub team: Option<String>,
    pub email: Option<String>,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
}
