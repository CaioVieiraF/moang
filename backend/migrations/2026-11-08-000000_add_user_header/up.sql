-- Banner (header image) shown by Mastodon and other ActivityPub clients.
ALTER TABLE users
    ADD COLUMN header_url TEXT NOT NULL DEFAULT '';
