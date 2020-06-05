table! {
    articles (id) {
        id -> Text,
        title -> Text,
        body -> Text,
    }
}

table! {
    labels (id) {
        id -> Integer,
        name -> Text,
        article_id -> Text,
    }
}

joinable!(labels -> articles (article_id));

allow_tables_to_appear_in_same_query!(articles, labels,);
