-- Add up migration script here
CREATE TABLE IF NOT EXISTS jobs
(
    job_id             UUID PRIMARY KEY,
    output_bucket_path TEXT         NOT NULL,
    status             VARCHAR(255) NOT NULL,
    video_id           UUID         NOT NULL,
    error              VARCHAR(255),
    created_at         TIMESTAMP    NOT NULL,
    updated_at         TIMESTAMP    NOT NULL,
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE ON UPDATE CASCADE
);