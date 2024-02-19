CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL CHECK (email LIKE '%@%'), -- Validate email format
  password CHAR(60) NOT NULL, -- Store hashed password using a secure algorithm
  profile_pic VARCHAR(256) DEFAULT NULL, -- Allow optional profile pic
  deleted_at BOOLEAN DEFAULT FALSE, -- Track deleted users efficiently
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() 
);

 
