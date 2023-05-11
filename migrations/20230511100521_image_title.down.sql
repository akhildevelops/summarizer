-- Add down migration script here

ALTER TABLE remoteurl DROP COLUMN title;

ALTER TABLE remoteurl DROP COLUMN image_id;