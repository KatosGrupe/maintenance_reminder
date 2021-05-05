table! {
    cameras (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    user_permissions (user_id, permission) {
        user_id -> Int4,
        permission -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

joinable!(user_permissions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cameras,
    user_permissions,
    users,
);
