table! {
    payments (id) {
        id -> Uuid,
        amount -> Numeric,
        from_user -> Uuid,
        to_user -> Uuid,
        created_at -> Timestamptz,
    }
}

table! {
    transfers (id) {
        id -> Uuid,
        amount -> Numeric,
        for_user -> Uuid,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(transfers -> users (for_user));

allow_tables_to_appear_in_same_query!(payments, transfers, users,);
