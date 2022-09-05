use crate::handlers::join::Info;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;

pub fn create_new_runner(form: &Info) -> NewRunner {
    NewRunner {
        firstname: Some(&form.firstname),
        lastname: Some(&form.lastname),
        team: Some(&form.team),
        email: Some(&form.email),
        starting_point: &form.starting_point,
        running_level: &form.running_level,
        donation: &form.donation,
    }
}

pub fn create_new_shipping(form: &Info, id: i32) -> NewShipping {
    NewShipping {
        tshirt_model: &form.tshirt_model,
        tshirt_size: &form.tshirt_size,
        country: &form.country,
        firstname: &form.address_firstname,
        lastname: &form.address_lastname,
        street_name: &form.street_name,
        house_number: &form.house_number,
        address_extra: Some(&form.address_extra),
        postal_code: &form.postal_code,
        city: &form.city,
        runner_id: id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_web::test]
    async fn create_new_runner_test() {
        let form = Info {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "somewhere".to_string(),
            running_level: "mediocre".to_string(),
            donation: "5".to_string(),
            ..Default::default()
        };
        let runner = create_new_runner(&form);
        assert_eq!(runner.firstname.unwrap(), form.firstname);
        assert_eq!(runner.lastname.unwrap(), form.lastname);
        assert_eq!(runner.team.unwrap(), form.team);
        assert_eq!(runner.email.unwrap(), form.email);
        assert_eq!(runner.starting_point, form.starting_point);
        assert_eq!(runner.running_level, form.running_level);
        assert_eq!(runner.donation, form.donation);
    }

    #[actix_web::test]
    async fn create_new_shipping_test() {
        let form = Info {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "somewhere".to_string(),
            running_level: "mediocre".to_string(),
            donation: "5".to_string(),
            tshirt_toggle: "".to_string(),
            tshirt_model: "unisex".to_string(),
            tshirt_size: "l".to_string(),
            country: "DE".to_string(),
            address_firstname: "Hans".to_string(),
            address_lastname: "Meyer".to_string(),
            street_name: "Street".to_string(),
            house_number: "1".to_string(),
            address_extra: "".to_string(),
            postal_code: "23455".to_string(),
            city: "Hamburg".to_string(),
            confirm: "on".to_string(),
        };
        let shipping = create_new_shipping(&form,1);
        assert_eq!(shipping.firstname, form.address_firstname);
        assert_eq!(shipping.lastname, form.address_lastname);
        assert_eq!(shipping.tshirt_model, form.tshirt_model);
        assert_eq!(shipping.tshirt_size, form.tshirt_size);
        assert_eq!(shipping.country, form.country);
        assert_eq!(shipping.street_name, form.street_name);
        assert_eq!(shipping.house_number, form.house_number);
        assert_eq!(shipping.postal_code, form.postal_code);
        assert_eq!(shipping.city, form.city);
    }
}
