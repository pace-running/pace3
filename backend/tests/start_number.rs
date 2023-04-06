use diesel::prelude::*;
use diesel::PgConnection;
use pace::constants::BLACKLIST_START_NUMBERS;
use pace::establish_connection;
use pace::models::start_number::next_start_number;

// For testing only
fn restart_start_number(conn: &mut PgConnection) {
    use diesel::sql_query;

    sql_query("ALTER SEQUENCE runner_start_number_seq RESTART")
        .execute(conn)
        .expect("Error resetting start_number sequence");
}

#[test]
fn integration_next_start_number_test_no_duplicates() {
    use std::collections::HashSet;

    let conn = &mut establish_connection();
    restart_start_number(conn);
    let mut generated = HashSet::new();

    for _ in 1..100 {
        let next = next_start_number(conn);
        assert!(!generated.contains(&next));
        generated.insert(next);
    }
}

#[test]
fn integration_next_start_number_test_no_blacklisted() {
    let conn = &mut establish_connection();
    restart_start_number(conn);
    for _ in 1..100 {
        let next = next_start_number(conn);
        assert!(!BLACKLIST_START_NUMBERS.contains(&next));
    }
}
