-- Connect to the database
\c mydb;

-- Create CryptoObject Table
CREATE TABLE IF NOT EXISTS CryptoObject (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL
);

-- Create MHObject Table without foreign key constraint
CREATE TABLE IF NOT EXISTS MHObject (
    id SERIAL PRIMARY KEY,
    mh_parameters TEXT NOT NULL,
    other_parameters TEXT,
    result_object_id INTEGER
);

-- Create ResultObject Table without foreign key constraint
CREATE TABLE IF NOT EXISTS ResultObject (
    id SERIAL PRIMARY KEY,
    results TEXT NOT NULL,
    other_parameters TEXT,
    mh_object_id INTEGER,
    crypto_object_id INTEGER UNIQUE
);

-- Create TaskObject Table
CREATE TABLE IF NOT EXISTS TaskObject (
    id SERIAL PRIMARY KEY,
    state VARCHAR(255) NOT NULL,
    other_parameters TEXT,
    mh_object_id INTEGER,
    crypto_object_id INTEGER,
    FOREIGN KEY (mh_object_id) REFERENCES MHObject(id),
    FOREIGN KEY (crypto_object_id) REFERENCES CryptoObject(id)
);

-- Add foreign key constraints to MHObject and ResultObject
ALTER TABLE MHObject
ADD CONSTRAINT fk_mh_object_result
FOREIGN KEY (result_object_id) REFERENCES ResultObject(id);

ALTER TABLE ResultObject
ADD CONSTRAINT fk_result_object_mh
FOREIGN KEY (mh_object_id) REFERENCES MHObject(id),
ADD CONSTRAINT fk_result_object_crypto
FOREIGN KEY (crypto_object_id) REFERENCES CryptoObject(id);
