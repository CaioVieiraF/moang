-- How a post is federated:
--   article   -> ActivityPub `Article` (excerpt + link to the frontend)
--   microblog -> ActivityPub `Note` (full content rendered by Mastodon)
ALTER TABLE posts
    ADD COLUMN kind VARCHAR(16) NOT NULL DEFAULT 'article';

ALTER TABLE posts
    ADD CONSTRAINT posts_kind_check CHECK (kind IN ('article', 'microblog'));
