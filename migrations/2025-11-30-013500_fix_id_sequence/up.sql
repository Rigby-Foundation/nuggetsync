-- Fix users table
CREATE SEQUENCE IF NOT EXISTS users_id_seq;
ALTER TABLE users ALTER COLUMN id SET DEFAULT nextval('users_id_seq');
ALTER SEQUENCE users_id_seq OWNED BY users.id;
SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 0) + 1, false);

-- Fix profiles table
CREATE SEQUENCE IF NOT EXISTS profiles_id_seq;
ALTER TABLE profiles ALTER COLUMN id SET DEFAULT nextval('profiles_id_seq');
ALTER SEQUENCE profiles_id_seq OWNED BY profiles.id;
SELECT setval('profiles_id_seq', COALESCE((SELECT MAX(id) FROM profiles), 0) + 1, false);
