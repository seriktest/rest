CREATE TABLE crates (
    id INTEGER PRIMARY KEY,
    rustacean_id INTEGER NOT NULL REFERENCES rustaceans(id),
    code VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL );
