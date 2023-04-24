// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    rejected_transactions (id) {
        id -> Int4,
        runner_ids -> Varchar,
        date_of_payment -> Varchar,
        reasons_for_payment -> Varchar,
        payment_amount -> Varchar,
        expected_amount -> Nullable<Varchar>,
        currency -> Varchar,
        payer_name -> Varchar,
        iban -> Varchar,
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
        payment_status -> Bool,
        verification_code -> Varchar,
        payment_confirmation_mail_sent -> Bool,
        tshirt_cost -> Varchar,
        bsv_participant -> Bool,
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
        delivery_status -> Varchar,
    }
}

diesel::table! {
    theme (event_key) {
        event_key -> Varchar,
        event_value -> Varchar,
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

diesel::allow_tables_to_appear_in_same_query!(
    events,
    rejected_transactions,
    runners,
    shippings,
    theme,
    users,
);
