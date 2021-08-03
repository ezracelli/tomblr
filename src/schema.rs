table! {
    blogs (id) {
        _rowid -> Int4,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        id -> Uuid,
        slug -> Text,
        title -> Text,
        updated_at -> Timestamptz,
        user_id -> Uuid,
    }
}

table! {
    email_accounts (id) {
        _rowid -> Int4,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        id -> Uuid,
        provider_account_id -> Text,
        updated_at -> Timestamptz,
        user_id -> Uuid,
    }
}

table! {
    oauth_accounts (id) {
        _rowid -> Int4,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        id -> Uuid,
        provider -> Text,
        provider_access_token -> Text,
        provider_access_token_expires_at -> Timestamptz,
        provider_account_id -> Text,
        provider_refresh_token -> Text,
        updated_at -> Timestamptz,
        user_id -> Uuid,
    }
}

table! {
    posts (id) {
        _rowid -> Int4,
        blog_id -> Uuid,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        id -> Uuid,
        slug -> Text,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        _rowid -> Int4,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        id -> Uuid,
        role -> Text,
        updated_at -> Timestamptz,
    }
}

joinable!(blogs -> users (user_id));
joinable!(email_accounts -> users (user_id));
joinable!(oauth_accounts -> users (user_id));
joinable!(posts -> blogs (blog_id));

allow_tables_to_appear_in_same_query!(
    blogs,
    email_accounts,
    oauth_accounts,
    posts,
    users,
);
