use diesel::prelude::*;
use diesel::sql_types::BigInt;

use crate::constants::BLACKLIST_START_NUMBERS;
use crate::get_next_start_number;

#[derive(QueryableByName)]
pub struct StartNumber {
    #[diesel(sql_type = BigInt)]
    pub start_number: i64,
}

pub fn next_start_number(conn: &mut PgConnection) -> i64 {
    let mut next = get_next_start_number(conn);

    while BLACKLIST_START_NUMBERS.contains(&next) {
        next = get_next_start_number(conn);
    }

    next
}
