-- This is just a dummy placeholder migration.
-- Normally there will be a snapshot of a real database schema.
-- Probably we will build it from a dump as a migration.
-- Then we can run this migration to seed the database with some data for tests.
PRAGMA FOREIGN_KEYS = OFF;

INSERT
OR IGNORE INTO posts (writer, content)
VALUES
    ('ali', 'Some more'),
    ('ali', 'posts.');