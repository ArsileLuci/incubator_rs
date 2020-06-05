table! {
    friends (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(friends, users,);
