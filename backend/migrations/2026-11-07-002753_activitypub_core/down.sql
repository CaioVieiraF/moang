DROP TABLE IF EXISTS remote_posts;
DROP TABLE IF EXISTS deliveries;
DROP TABLE IF EXISTS activities;
DROP TABLE IF EXISTS following;
DROP TABLE IF EXISTS followers;
DROP TABLE IF EXISTS remote_actors;
DROP TABLE IF EXISTS actor_keys;

DROP INDEX IF EXISTS posts_ap_id_unique;
ALTER TABLE posts
    DROP COLUMN IF EXISTS visibility,
    DROP COLUMN IF EXISTS updated_at,
    DROP COLUMN IF EXISTS ap_id;

DROP INDEX IF EXISTS users_name_unique;
ALTER TABLE users
    DROP COLUMN IF EXISTS is_discoverable,
    DROP COLUMN IF EXISTS is_locked,
    DROP COLUMN IF EXISTS created_at,
    DROP COLUMN IF EXISTS avatar_url,
    DROP COLUMN IF EXISTS summary,
    DROP COLUMN IF EXISTS display_name;
