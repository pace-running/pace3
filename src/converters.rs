use crate::get_next_start_number;
use crate::models::info::Info;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;
use diesel::PgConnection;

const BLACKLIST_START_NUMBERS: [i64; 20] = [
    18, 28, 33, 45, 74, 84, 88, 444, 191, 192, 198, 420, 1312, 1717, 1887, 1910, 1919, 1933, 1488,
    1681,
];

pub fn create_new_runner<'a>(form: &'a Info, conn: &mut PgConnection) -> NewRunner<'a> {
    let start_number = next_start_number(conn);

    NewRunner {
        start_number,
        firstname: Some(&form.runner_info.firstname),
        lastname: Some(&form.runner_info.lastname),
        team: Some(&form.runner_info.team),
        email: Some(&form.runner_info.email),
        starting_point: &form.runner_info.starting_point,
        running_level: &form.runner_info.running_level,
        donation: &form.runner_info.donation,
    }
}

pub fn create_new_shipping(form: &Info, id: i32) -> NewShipping {
    NewShipping {
        tshirt_model: &form.shipping_info.tshirt_model,
        tshirt_size: &form.shipping_info.tshirt_size,
        country: &form.shipping_info.country,
        firstname: &form.shipping_info.address_firstname,
        lastname: &form.shipping_info.address_lastname,
        street_name: &form.shipping_info.street_name,
        house_number: &form.shipping_info.house_number,
        address_extra: Some(&form.shipping_info.address_extra),
        postal_code: &form.shipping_info.postal_code,
        city: &form.shipping_info.city,
        runner_id: id,
    }
}

fn next_start_number(conn: &mut PgConnection) -> i64 {
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
        let form = InfoBuilder::minimal_default().build();
        let conn = &mut establish_connection();
        let runner = create_new_runner(&form, conn);

        assert_eq!(runner.firstname.unwrap(), form.runner_info.firstname);
        assert_eq!(runner.lastname.unwrap(), form.runner_info.lastname);
        assert_eq!(runner.team.unwrap(), form.runner_info.team);
        assert_eq!(runner.email.unwrap(), form.runner_info.email);
        assert_eq!(runner.starting_point, form.runner_info.starting_point);
        assert_eq!(runner.running_level, form.runner_info.running_level);
        assert_eq!(runner.donation, form.runner_info.donation);
        assert!(runner.start_number > 3 && !BLACKLIST_START_NUMBERS.contains(&runner.start_number))
    }

    #[actix_web::test]
    async fn create_new_shipping_test() {
        let form = InfoBuilder::default().build();
        let shipping = create_new_shipping(&form, 1);
        assert_eq!(shipping.firstname, form.shipping_info.address_firstname);
        assert_eq!(shipping.lastname, form.shipping_info.address_lastname);
        assert_eq!(shipping.tshirt_model, form.shipping_info.tshirt_model);
        assert_eq!(shipping.tshirt_size, form.shipping_info.tshirt_size);
        assert_eq!(shipping.country, form.shipping_info.country);
        assert_eq!(shipping.street_name, form.shipping_info.street_name);
        assert_eq!(shipping.house_number, form.shipping_info.house_number);
        assert_eq!(shipping.postal_code, form.shipping_info.postal_code);
        assert_eq!(shipping.city, form.shipping_info.city);
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
