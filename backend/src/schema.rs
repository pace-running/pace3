// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

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
        reason_for_payment -> Varchar,
        status_link -> Varchar,
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

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
    }
}

diesel::joinable!(shippings -> runners (runner_id));

diesel::allow_tables_to_appear_in_same_query!(events, runners, shippings, users,);
