PRAGMA FOREIGN_KEYS = OFF;

-- Important:
--
-- This migration is neither a suggestion for a db schema nor related to what this project is about.
-- The sole purpose is to start with something and lay the ground work in the codebase.
-- It wil be overridden by the first real migration.
CREATE TABLE IF NOT EXISTS posts (writer TEXT NOT NULL, content TEXT NOT NULL);

INSERT
    OR IGNORE INTO posts (writer, content)
VALUES
    ('ali', 'I don''t know what I''m doing!'),
    ('ozan', 'I''m good with namings!'),
    ('caner', 'I write a lot into the group!');