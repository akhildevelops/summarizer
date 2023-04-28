-- Add up migration script here

ALTER TABLE transcript ALTER COLUMN content TYPE TEXT;

ALTER TABLE transcriptsummary ALTER COLUMN content TYPE TEXT;