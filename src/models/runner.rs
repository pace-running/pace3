use super::info::Info;
use crate::get_next_start_number;
use crate::schema::runners;
use diesel::prelude::*;
use diesel::PgConnection;

const BLACKLIST_START_NUMBERS: [i64; 20] = [
    18, 28, 33, 45, 74, 84, 88, 444, 191, 192, 198, 420, 1312, 1717, 1887, 1910, 1919, 1933, 1488,
    1681,
];
#[derive(Insertable)]
#[diesel(table_name = runners)]
pub struct NewRunner<'a> {
    pub start_number: i64,
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
    pub start_number: i64,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub team: Option<String>,
    pub email: Option<String>,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
}

impl<'a> From<(&'a Info, i64)> for NewRunner<'a> {
    fn from(info_with_start_number: (&'a Info, i64)) -> Self {
        let (info, next_start_number) = info_with_start_number;

        NewRunner {
            start_number: next_start_number,
            firstname: Some(&info.runner_info.firstname),
            lastname: Some(&info.runner_info.lastname),
            team: Some(&info.runner_info.team),
            email: Some(&info.runner_info.email),
            starting_point: &info.runner_info.starting_point,
            running_level: &info.runner_info.running_level,
            donation: &info.runner_info.donation,
        }
    }
}

pub fn next_start_number(conn: &mut PgConnection) -> i64 {
    let mut next = get_next_start_number(conn);

    while BLACKLIST_START_NUMBERS.contains(&next) {
        next = get_next_start_number(conn);
    }

    return next;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::InfoBuilder;
    use crate::establish_connection;

    #[actix_web::test]
    async fn create_new_runner_test() {
        let info = InfoBuilder::minimal_default().build();
        let expected_start_number = 10;
        let runner = NewRunner::from((&info, expected_start_number));

        assert_eq!(runner.firstname.unwrap(), info.runner_info.firstname);
        assert_eq!(runner.lastname.unwrap(), info.runner_info.lastname);
        assert_eq!(runner.team.unwrap(), info.runner_info.team);
        assert_eq!(runner.email.unwrap(), info.runner_info.email);
        assert_eq!(runner.starting_point, info.runner_info.starting_point);
        assert_eq!(runner.running_level, info.runner_info.running_level);
        assert_eq!(runner.donation, info.runner_info.donation);
        assert_eq!(runner.start_number, expected_start_number);
    }

    #[test]
    fn next_start_number_test_no_duplicates() {
        use crate::restart_start_number;
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
    fn next_start_number_test_no_blacklisted() {
        use crate::restart_start_number;

        let conn = &mut establish_connection();
        restart_start_number(conn);
        for _ in 1..100 {
            let next = next_start_number(conn);
            assert!(!BLACKLIST_START_NUMBERS.contains(&next));
        }
    }
}
