-- Connect to the database
\c mydb;

-- Create crypto_symbol Table
CREATE TABLE IF NOT EXISTS crypto_symbol (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    volume FLOAT NOT NULL,
    last_updated TIMESTAMP NOT NULL,
    available BOOLEAN DEFAULT TRUE,
    UNIQUE (symbol)
);

-- Create crypto_interval ENUM type
CREATE TYPE crypto_interval AS ENUM ('Int1m', 'Int5m', 'Int15m', 'Int30m', 'Int1h', 'Int2h', 'Int4h', 'Int6h', 'Int8h', 'Int12h', 'Int1d', 'Int3d', 'Int1w', 'Int1M');

-- Create crypto_list Table
CREATE TABLE IF NOT EXISTS crypto_list (
    id SERIAL PRIMARY KEY,
    hidden BOOLEAN DEFAULT FALSE NOT NULL,
    name VARCHAR(255) NOT NULL,
    interval crypto_interval NOT NULL,
    limit_count INTEGER NOT NULL,
    type VARCHAR(255) NOT NULL
);

-- Create junction table for many-to-many relationship between crypto_list and crypto_symbol
CREATE TABLE IF NOT EXISTS crypto_list_x_crypto_symbol (
    crypto_list_id INTEGER NOT NULL,
    crypto_symbol_id INTEGER NOT NULL,
    PRIMARY KEY (crypto_list_id, crypto_symbol_id),
    FOREIGN KEY (crypto_list_id) REFERENCES crypto_list(id),
    FOREIGN KEY (crypto_symbol_id) REFERENCES crypto_symbol(id)
);

-- Create mh_object Table without foreign key constraint
CREATE TABLE IF NOT EXISTS mh_object (
    id SERIAL PRIMARY KEY,
    hidden BOOLEAN DEFAULT FALSE NOT NULL,
    mh_algorithm_name TEXT NOT NULL,
    mh_parameters TEXT NOT NULL,
    other_parameters TEXT
);

-- Create result Table without foreign key constraint
CREATE TABLE IF NOT EXISTS result (
    id SERIAL PRIMARY KEY,
    results TEXT NOT NULL,
    other_parameters TEXT
);

-- Create state ENUM type
CREATE TYPE state_enum AS ENUM ('CREATED', 'PENDING', 'RUNNING', 'CANCELLING', 'CANCELLED', 'COMPLETED', 'FAILED');

-- Create task Table
CREATE TABLE IF NOT EXISTS task (
    id SERIAL PRIMARY KEY,
    state state_enum NOT NULL DEFAULT 'CREATED',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    other_parameters TEXT,
    mh_object_id INTEGER,
    crypto_list_id INTEGER,
    indicator_combination_id INTEGER,
    result_id INTEGER,
    FOREIGN KEY (mh_object_id) REFERENCES mh_object(id),
    FOREIGN KEY (crypto_list_id) REFERENCES crypto_list(id),
    FOREIGN KEY (result_id) REFERENCES result(id)
);

-- Create indicator combination Table
CREATE TABLE IF NOT EXISTS indicator_combination (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    hidden BOOLEAN DEFAULT FALSE NOT NULL
);

-- Create indicator in combination Table
CREATE TABLE IF NOT EXISTS indicator_in_combination (
    id SERIAL PRIMARY KEY,
    indicator_combination_id INTEGER NOT NULL,
    indicator_struct_name TEXT NOT NULL,
    parameters TEXT NOT NULL,
    FOREIGN KEY (indicator_combination_id) REFERENCES indicator_combination(id)
);


---------------------------------------------------------------


-- Insert data into crypto_symbol Table
INSERT INTO crypto_symbol (symbol, name, volume, last_updated)
VALUES ('BTC', 'Bitcoin', 1.0, '2021-01-01 00:00:00'),
       ('ETH', 'Ethereum', 0.5, '2021-01-01 00:00:00'),
       ('XRP', 'Ripple', 0.3, '2021-01-01 00:00:00');

-- Insert data into crypto_list Table
INSERT INTO crypto_list (name, type, interval, limit_count)
VALUES ('Top 2', 'Top', 'Int1h', 20);

-- Insert data into crypto_list_x_crypto_symbol Table
INSERT INTO crypto_list_x_crypto_symbol (crypto_list_id, crypto_symbol_id)
VALUES (1, 1),
       (1, 2);

-- Show crypto symbol for the crypto list 'Top 2'
SELECT cs.symbol, cs.name
FROM crypto_symbol cs
JOIN crypto_list_x_crypto_symbol clxcs
ON cs.id = clxcs.crypto_symbol_id
JOIN crypto_list cl
ON cl.id = clxcs.crypto_list_id
WHERE cl.name = 'Top 2';

-- Insert data into mh_object Table
INSERT INTO mh_object (mh_algorithm_name, mh_parameters, other_parameters)
VALUES ('mh_1', '{"param1": "value1"}', '{"other_param1": "other_value1"}'),
       ('mh_2', '{"param2": "value2"}', '{"other_param2": "other_value2"}');

-- Insert data into result Table
INSERT INTO result (results, other_parameters)
VALUES ('{"result1": "result_value1"}', '{"other_result1": "other_result_value1"}'),
       ('{"result2": "result_value2"}', '{"other_result2": "other_result_value2"}');

-- Insert data into task Table
INSERT INTO task (state, other_parameters, mh_object_id, crypto_list_id, result_id)
VALUES ('PENDING', '{"task_param1": "task_value1"}', 1, 1, NULL),
       ('COMPLETED', '{"task_param2": "task_value2"}', 2, 1, 2);

-- Show tasks with mh_object and result information
SELECT t.id, t.state, t.other_parameters, mh.mh_parameters, r.results
FROM task t
LEFT JOIN mh_object mh
ON t.mh_object_id = mh.id
LEFT JOIN result r
ON t.result_id = r.id;

-- Update task state to 'COMPLETED' and set result_id to 1
UPDATE task
SET state = 'FAILED',
    result_id = 1
WHERE id = 1;

-- Show all tasks for the crypto list 'Top 2'
SELECT t.id, t.state, t.other_parameters, mh.mh_parameters, r.results
FROM task t
LEFT JOIN mh_object mh
ON t.mh_object_id = mh.id
LEFT JOIN result r
ON t.result_id = r.id
WHERE t.crypto_list_id = 1;

-- Show all crypto symbols
SELECT *
FROM crypto_symbol;

-- Show tables
\dt
