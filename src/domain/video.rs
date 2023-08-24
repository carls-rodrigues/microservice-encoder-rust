use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct Video {
    id: String,
    resource_id: String,
    file_path: String,
    created_at: NaiveDateTime,
}

impl Video {
    pub fn new(id: String, resource_id: String, file_path: String, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            resource_id,
            file_path,
            created_at,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() || Uuid::parse_str(&self.id).is_err() {
            return Err(String::from("Id is empty or invalid"));
        }
        if self.resource_id.is_empty() || Uuid::parse_str(&self.resource_id).is_err() {
            return Err(String::from("Resource id is empty or invalid"));
        }
        if self.file_path.is_empty() {
            return Err(String::from("File path is empty"));
        }
        Ok(())
    }
}