// @generated automatically by Diesel CLI.

diesel::table! {
    article_categories (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    articles (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        link -> Varchar,
        pub_date -> Timestamptz,
        #[max_length = 255]
        media -> Nullable<Varchar>,
        content -> Text,
        #[max_length = 255]
        creator -> Varchar,
        feed_id -> Uuid,
        ai_summary -> Nullable<Text>,
        rate -> Nullable<Int4>,
        #[max_length = 255]
        keywords -> Nullable<Varchar>,
        processing_ai_summary -> Bool,
        processing_rating -> Bool,
        processing_keywords -> Bool,
        category_id -> Nullable<Uuid>,
        processing_categorizing -> Bool,
        ai_columnist -> Nullable<Text>,
        processing_columnist -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    feeds (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        feed_url -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        link -> Varchar,
        last_build_date -> Nullable<Timestamptz>,
        #[max_length = 10]
        language -> Nullable<Varchar>,
        #[max_length = 10]
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(articles -> article_categories (category_id));
diesel::joinable!(articles -> feeds (feed_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_categories,
    articles,
    feeds,
    users,
);

