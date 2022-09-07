use diesel::prelude::*;
use diesel::sql_types::BigInt;

#[derive(QueryableByName)]
pub struct StartNumber {
    #[diesel(sql_type = BigInt)]
    pub start_number: i64
}
