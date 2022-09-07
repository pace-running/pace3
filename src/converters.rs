use crate::models::info::Info;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;

pub fn create_new_runner(form: &Info) -> NewRunner {
    NewRunner {
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

#[cfg(test)]
mod tests {
    use crate::builders::InfoBuilder;

    use super::*;

    #[actix_web::test]
    async fn create_new_runner_test() {
        let form = InfoBuilder::minimal_default().build();
        let runner = create_new_runner(&form);
        assert_eq!(runner.firstname.unwrap(), form.runner_info.firstname);
        assert_eq!(runner.lastname.unwrap(), form.runner_info.lastname);
        assert_eq!(runner.team.unwrap(), form.runner_info.team);
        assert_eq!(runner.email.unwrap(), form.runner_info.email);
        assert_eq!(runner.starting_point, form.runner_info.starting_point);
        assert_eq!(runner.running_level, form.runner_info.running_level);
        assert_eq!(runner.donation, form.runner_info.donation);
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
}
