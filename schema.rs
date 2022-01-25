table! {
    reviews (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        hospital_name -> Text,
        doctor_name -> Text,
        doctor_type -> Text,
        service_rendered -> Text,
        service_cost -> Int8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        passwd -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    reviews,
    users,
);
