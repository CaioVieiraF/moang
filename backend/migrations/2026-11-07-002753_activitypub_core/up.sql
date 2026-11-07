-- ActivityPub core data.
-- Kept in one migration because the tables are only useful together.

CREATE EXTENSION IF NOT EXISTS pgcrypto;

ALTER TABLE users
    ADD COLUMN display_name VARCHAR NOT NULL DEFAULT '',
    ADD COLUMN summary TEXT NOT NULL DEFAULT '',
    ADD COLUMN avatar_url TEXT NOT NULL DEFAULT '',
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    ADD COLUMN is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN is_discoverable BOOLEAN NOT NULL DEFAULT TRUE;

CREATE UNIQUE INDEX users_name_unique ON users (name);

ALTER TABLE posts
    ADD COLUMN ap_id UUID NOT NULL DEFAULT gen_random_uuid(),
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    ADD COLUMN visibility VARCHAR(16) NOT NULL DEFAULT 'public';

UPDATE posts
SET visibility = CASE WHEN is_public THEN 'public' ELSE 'private' END;

CREATE UNIQUE INDEX posts_ap_id_unique ON posts (ap_id);

CREATE TABLE actor_keys (
    user_id VARCHAR(36) PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    public_key_pem TEXT NOT NULL,
    private_key_pem TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE remote_actors (
    id TEXT PRIMARY KEY,
    preferred_username VARCHAR NOT NULL DEFAULT '',
    domain VARCHAR NOT NULL,
    name VARCHAR NOT NULL DEFAULT '',
    summary TEXT NOT NULL DEFAULT '',
    avatar_url TEXT NOT NULL DEFAULT '',
    inbox_url TEXT NOT NULL,
    shared_inbox_url TEXT,
    public_key_id TEXT NOT NULL,
    public_key_pem TEXT NOT NULL,
    fetched_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE followers (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_id TEXT NOT NULL REFERENCES remote_actors(id) ON DELETE CASCADE,
    accepted BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, actor_id)
);

CREATE TABLE following (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_id TEXT NOT NULL REFERENCES remote_actors(id) ON DELETE CASCADE,
    accepted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, actor_id)
);

CREATE TABLE activities (
    id TEXT PRIMARY KEY,
    user_id VARCHAR(36) REFERENCES users(id) ON DELETE CASCADE,
    actor_id TEXT NOT NULL,
    activity_type VARCHAR NOT NULL,
    object_id TEXT,
    direction VARCHAR(8) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE deliveries (
    id SERIAL PRIMARY KEY,
    activity_id TEXT NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    inbox_url TEXT NOT NULL,
    state VARCHAR(16) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    next_attempt_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_error TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (activity_id, inbox_url)
);

CREATE TABLE remote_posts (
    id TEXT PRIMARY KEY,
    actor_id TEXT NOT NULL REFERENCES remote_actors(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    summary TEXT,
    published TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    url TEXT,
    in_reply_to TEXT,
    sensitive BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX remote_posts_actor_idx ON remote_posts (actor_id);
CREATE INDEX remote_posts_in_reply_to_idx ON remote_posts (in_reply_to);
