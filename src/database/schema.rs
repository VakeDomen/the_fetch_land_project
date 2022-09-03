table! {
    sales (id) {
        id -> Text,
        sale_type -> Text,
        user_id -> Text,
        sale_object_id -> Text,
        location_coords -> Text,
        created -> Text,
        price -> Integer,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    sales,
    users,
);
