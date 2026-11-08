ALTER TABLE remote_posts
    DROP COLUMN IF EXISTS replies_count,
    DROP COLUMN IF EXISTS reposts_count,
    DROP COLUMN IF EXISTS likes_count;

DROP TABLE IF EXISTS remote_interactions;
