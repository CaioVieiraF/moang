ALTER TABLE remote_posts
    DROP COLUMN IF EXISTS cc_addresses,
    DROP COLUMN IF EXISTS to_addresses;
