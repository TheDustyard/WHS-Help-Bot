table! {
    classes (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        classes -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    classes,
    users,
);
