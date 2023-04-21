-- Add up migration script here
CREATE TYPE summary_agent AS ENUM ('openai','other');

CREATE TABLE IF NOT EXISTS REMOTEURL(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    link VARCHAR
);

CREATE TABLE IF NOT EXISTS TRANSCRIPT(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    remote_id INT REFERENCES REMOTEURL ON DELETE RESTRICT, -- Doesn't delete YOUTUBE(ID) if it still has a reference to youtube_id
    content VARCHAR
);


CREATE TABLE IF NOT EXISTS TRANSCRIPTSUMMARY(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transcript_id INT REFERENCES TRANSCRIPT ON DELETE RESTRICT, -- Doesn't deletes Transcript(ID) if it has a reference to TranscriptSummary table
    content VARCHAR,
    agent SUMMARY_AGENT default 'other'
);