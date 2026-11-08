-- Audience of remote posts, kept so the replies collection can be exposed as
-- valid ActivityPub objects (with `to` and `cc`).
ALTER TABLE remote_posts
    ADD COLUMN to_addresses JSONB NOT NULL DEFAULT '[]'::jsonb,
    ADD COLUMN cc_addresses JSONB NOT NULL DEFAULT '[]'::jsonb;
