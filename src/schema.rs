// @generated automatically by Diesel CLI.

diesel::table! {
    bird (id) {
        id -> Integer,
        name -> Varchar,
        scientific_name -> Varchar,
        commonwealth_status -> Varchar,
    }
}

diesel::table! {
    bird_sighting (id) {
        id -> Integer,
        bird_id -> Integer,
        sighting_date -> Nullable<Timestamp>,
        sighting_location -> Nullable<Varchar>,
        additional_information -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bird,
    bird_sighting,
);
