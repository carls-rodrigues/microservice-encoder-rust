use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::Job;

#[derive(Clone, Debug)]
pub struct Video<'a> {
    id: String,
    resource_id: String,
    file_path: String,
    created_at: NaiveDateTime,
    jobs: Vec<&'a Job<'a>>,
}

impl<'a> Video<'a> {
    pub fn new(
        id: String,
        resource_id: String,
        file_path: String,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            resource_id,
            file_path,
            created_at,
            jobs: Vec::new(),
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
    pub fn id(&self) -> Uuid {
        Uuid::parse_str(&self.id).unwrap()
    }
    pub fn resource_id(&self) -> Uuid {
        Uuid::parse_str(&self.resource_id).unwrap()
    }
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    pub fn jobs(&self) -> &Vec<&'a Job<'a>> {
        &self.jobs
    }
}
