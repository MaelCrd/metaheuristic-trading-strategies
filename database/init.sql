-- Create extensions and initial setup
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- You can add your initial schema and data here
-- For example:
-- CREATE TABLE IF NOT EXISTS users (
--     id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
--     username VARCHAR(50) UNIQUE NOT NULL,
--     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
-- );