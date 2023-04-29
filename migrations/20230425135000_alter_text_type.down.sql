-- Add down migration script here

ALTER TABLE transcript ALTER COLUMN content TYPE VARCHAR;

ALTER TABLE transcriptsummary ALTER COLUMN content TYPE VARCHAR;