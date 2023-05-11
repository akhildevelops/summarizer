-- Add up migration script here

ALTER TABLE remoteurl ADD COLUMN title VARCHAR;

ALTER TABLE remoteurl ADD COLUMN image_id VARCHAR;