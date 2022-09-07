use diesel::prelude::*;
use diesel::sql_types::Int4;

#[derive(QueryableByName)]
pub struct StartNumber {
    #[diesel(sql_type = Int4)]
    pub start_number: i32
}
