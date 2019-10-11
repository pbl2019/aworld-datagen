table! {
    characters (id) {
        id -> Bigint,
        name -> Char,
        max_hp -> Integer,
        max_appetite -> Integer,
    }
}

table! {
    terrains (id) {
        id -> Bigint,
        content -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    characters,
    terrains,
);
