use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::video::Video;

pub struct Job<'a> {
    id: String,
    output_bucket_path: String,
    status: String,
    video: &'a Video,
    video_id: Option<String>,
    error: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

impl<'a> Job<'a> {
    pub fn new(
        output: String,
        status: String,
        video: &'a Video
    ) -> Result<Self, String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        Ok(Self {
            id,
            output_bucket_path: output,
            status,
            video,
            video_id: None,
            error: None,
            created_at: now,
            updated_at: now
        })
    }

    pub fn validate(&self, job: &Job) -> Result<(), String> {
        if job.id.is_empty() || Uuid::parse_str(&job.id).is_err() {
            return Err(String::from("Job Id is empty or invalid"));
        }
        if job.output_bucket_path.is_empty() {
            return Err(String::from("Output bucket path is empty"));
        }
        if job.status.is_empty() {
            return Err(String::from("Status is empty"));
        }
        Ok(())
    }
}