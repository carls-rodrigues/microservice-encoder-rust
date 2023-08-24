use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::video::Video;

#[derive(Debug, Clone)]
pub struct Job<'a> {
    id: String,
    output_bucket_path: String,
    status: String,
    video: &'a Video<'a>,
    video_id: String,
    error: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl<'a> Job<'a> {
    pub fn new(
        output: String,
        status: String,
        video: &'a Video,
        video_id: String,
    ) -> Result<Self, String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        Ok(Self {
            id,
            output_bucket_path: output,
            status,
            video,
            video_id,
            error: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn from(
        id: String,
        output: String,
        status: String,
        video: &'a Video,
        video_id: String,
        error: Option<String>,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Result<Self, String> {
        Ok(Self {
            id,
            output_bucket_path: output,
            status,
            video,
            video_id,
            error,
            created_at,
            updated_at,
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
        if job.video_id.is_empty() || Uuid::parse_str(&job.video_id).is_err() {
            return Err(String::from("Video id is empty or invalid"));
        }
        Ok(())
    }
    pub fn id(&self) -> Uuid {
        Uuid::parse_str(&self.id).unwrap()
    }
    pub fn output_bucket_path(&self) -> &str {
        &self.output_bucket_path
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn video(&self) -> &'a Video<'a> {
        self.video
    }
    pub fn video_id(&self) -> Uuid {
        Uuid::parse_str(&self.video_id).unwrap()
    }
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
