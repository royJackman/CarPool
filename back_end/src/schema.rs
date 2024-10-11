// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    reservations (id) {
        id -> Integer,
        description -> Nullable<Text>,
        startTime -> Integer,
        endTime -> Integer,
        userId -> Integer,
        carId -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    reservations,
    users,
);
