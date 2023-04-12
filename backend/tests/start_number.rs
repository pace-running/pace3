use diesel::prelude::*;
use diesel::PgConnection;
use pace::constants::DENYLIST_START_NUMBERS;
use pace::get_connection_pool;
use pace::models::start_number::next_start_number;

// For testing only
fn restart_start_number(conn: &mut PgConnection) {
    use diesel::sql_query;

    sql_query("ALTER SEQUENCE runner_start_number_seq RESTART")
        .execute(conn)
        .expect("Error resetting start_number sequence");
}

#[test]
fn next_start_number_does_not_generate_duplicates_or_deny_listed_start_numbers() {
    use std::collections::HashSet;

    let conn = &mut get_connection_pool()
        .expect("Unable to get connection pool.")
        .get()
        .expect("Unable to get connection.");
    restart_start_number(conn);
    let mut generated = HashSet::new();

    for _ in 1..100 {
        let next = next_start_number(conn);
        assert!(!generated.contains(&next));
        generated.insert(next);
    }

    assert!(generated.is_disjoint(&HashSet::from(DENYLIST_START_NUMBERS)))
}
