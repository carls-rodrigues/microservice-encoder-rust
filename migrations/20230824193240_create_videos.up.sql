-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos
(
    id          UUID         not null PRIMARY KEY,
    resource_id UUID         not null,
    file_path   VARCHAR(255) not null,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);