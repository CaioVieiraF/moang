// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Text,
        #[max_length = 36]
        user_id -> Nullable<Varchar>,
        actor_id -> Text,
        activity_type -> Varchar,
        object_id -> Nullable<Text>,
        #[max_length = 8]
        direction -> Varchar,
        payload -> Jsonb,
        created_at -> Timestamp,
    }
}

diesel::table! {
    actor_keys (user_id) {
        #[max_length = 36]
        user_id -> Varchar,
        public_key_pem -> Text,
        private_key_pem -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    deliveries (id) {
        id -> Int4,
        activity_id -> Text,
        inbox_url -> Text,
        #[max_length = 16]
        state -> Varchar,
        attempts -> Int4,
        next_attempt_at -> Timestamp,
        last_error -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    followers (id) {
        id -> Int4,
        #[max_length = 36]
        user_id -> Varchar,
        actor_id -> Text,
        accepted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    following (id) {
        id -> Int4,
        #[max_length = 36]
        user_id -> Varchar,
        actor_id -> Text,
        accepted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        is_public -> Bool,
        slug -> Varchar,
        #[max_length = 36]
        author -> Varchar,
        created_at -> Timestamp,
        ap_id -> Uuid,
        updated_at -> Timestamp,
        #[max_length = 16]
        visibility -> Varchar,
        #[max_length = 16]
        kind -> Varchar,
    }
}

diesel::table! {
    remote_actors (id) {
        id -> Text,
        preferred_username -> Varchar,
        domain -> Varchar,
        name -> Varchar,
        summary -> Text,
        avatar_url -> Text,
        inbox_url -> Text,
        shared_inbox_url -> Nullable<Text>,
        public_key_id -> Text,
        public_key_pem -> Text,
        fetched_at -> Timestamp,
    }
}

diesel::table! {
    remote_posts (id) {
        id -> Text,
        actor_id -> Text,
        content -> Text,
        summary -> Nullable<Text>,
        published -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        url -> Nullable<Text>,
        in_reply_to -> Nullable<Text>,
        sensitive -> Bool,
        created_at -> Timestamp,
        likes_count -> Int4,
        reposts_count -> Int4,
        replies_count -> Int4,
        to_addresses -> Jsonb,
        cc_addresses -> Jsonb,
    }
}

diesel::table! {
    remote_interactions (id) {
        id -> Text,
        actor_id -> Text,
        object_id -> Text,
        #[max_length = 16]
        interaction_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
        display_name -> Varchar,
        summary -> Text,
        avatar_url -> Text,
        header_url -> Text,
        created_at -> Timestamp,
        is_locked -> Bool,
        is_discoverable -> Bool,
    }
}

diesel::joinable!(activities -> users (user_id));
diesel::joinable!(actor_keys -> users (user_id));
diesel::joinable!(deliveries -> activities (activity_id));
diesel::joinable!(followers -> remote_actors (actor_id));
diesel::joinable!(followers -> users (user_id));
diesel::joinable!(following -> remote_actors (actor_id));
diesel::joinable!(following -> users (user_id));
diesel::joinable!(posts -> users (author));
diesel::joinable!(remote_posts -> remote_actors (actor_id));
diesel::joinable!(remote_interactions -> remote_actors (actor_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    actor_keys,
    deliveries,
    followers,
    following,
    posts,
    remote_actors,
    remote_interactions,
    remote_posts,
    users,
);
