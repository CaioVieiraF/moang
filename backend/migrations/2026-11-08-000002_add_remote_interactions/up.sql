-- Likes and reposts received from the fediverse for local posts.
CREATE TABLE remote_interactions (
    id TEXT PRIMARY KEY,
    actor_id TEXT NOT NULL REFERENCES remote_actors(id) ON DELETE CASCADE,
    object_id TEXT NOT NULL,
    interaction_type VARCHAR(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (actor_id, object_id, interaction_type)
);

CREATE INDEX remote_interactions_object_idx
    ON remote_interactions (object_id, interaction_type);

-- Counts reported by the origin server for remote posts, shown in the UI.
ALTER TABLE remote_posts
    ADD COLUMN likes_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN reposts_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN replies_count INTEGER NOT NULL DEFAULT 0;
