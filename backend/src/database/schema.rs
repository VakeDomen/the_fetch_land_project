table! {
    sales (id) {
        id -> Text,
        sale_type -> Text,
        user_id -> Text,
        sale_object_id -> Text,
        created -> Text,
        description -> Text,
        price -> Integer,
        amount -> Integer,
        contact_type -> Text,
        location -> Text,
        web_address -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        google_id -> Text,
        img -> Text,
        email -> Text,
        name -> Text,
        phone -> Text,
    }
}

table! {
    subscriptions (id) {
        id -> Text,
        sale_type -> Text,
        user_id -> Text,
        sale_object_id -> Text,
        created -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    sales,
    users,
);
