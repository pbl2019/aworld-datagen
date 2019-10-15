table! {
    characters (id) {
        id -> Bigint,
        name -> Char,
        max_hp -> Integer,
        max_appetite -> Integer,
    }
}

table! {
    items (id) {
        id -> Bigint,
        name -> Char,
        item_type -> Integer,
        amount -> Bigint,
    }
}

table! {
    relations (id) {
        id -> Bigint,
        character_id -> Bigint,
        target_id -> Bigint,
        factor -> Double,
    }
}

table! {
    terrains (id) {
        id -> Bigint,
        content -> Text,
        width -> Integer,
        height -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    characters,
    items,
    relations,
    terrains,
);
