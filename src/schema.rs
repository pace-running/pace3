// @generated automatically by Diesel CLI.

diesel::table! {
    runners (id) {
        id -> Int4,
        start_number -> Int8,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        team -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        starting_point -> Varchar,
        running_level -> Varchar,
        donation -> Varchar,
    }
}

diesel::table! {
    shippings (id) {
        id -> Int4,
        tshirt_model -> Varchar,
        tshirt_size -> Varchar,
        country -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        street_name -> Varchar,
        house_number -> Varchar,
        address_extra -> Nullable<Varchar>,
        postal_code -> Varchar,
        city -> Varchar,
        runner_id -> Int4,
    }
}

diesel::joinable!(shippings -> runners (runner_id));

diesel::allow_tables_to_appear_in_same_query!(runners, shippings,);
