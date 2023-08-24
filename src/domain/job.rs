use chrono::NaiveDateTime;
use crate::domain::video::Video;

pub struct Job<'a> {
    id: String,
    output_bucket_path: String,
    status: String,
    video: &'a Video,
    error: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}