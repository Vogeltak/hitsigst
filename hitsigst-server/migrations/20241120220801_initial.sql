CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY,
    title TEXT,
    artist TEXT,
    year INT,
    uploaded_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
