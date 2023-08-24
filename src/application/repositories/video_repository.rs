use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::Video;

#[async_trait]
pub trait VideoRepository<'a> {
    async fn insert(&self, video: &'a Video<'a>) -> Result<Video<'a>, String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Video<'a>, String>;
}

pub struct VideoRepositoryImplementation {
    connection: PgPool,
}

impl VideoRepositoryImplementation {
    pub fn instance(connection: &PgPool) -> Self {
        Self {
            connection: connection.clone(),
        }
    }
}

#[async_trait]
impl<'a> VideoRepository<'a> for VideoRepositoryImplementation {
    async fn insert(&self, video: &'a Video<'a>) -> Result<Video<'a>, String> {
        return match sqlx::query(
            r#"
                INSERT INTO videos (id, resource_id, file_path, created_at)
                VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&video.id())
        .bind(&video.resource_id())
        .bind(&video.file_path())
        .bind(&video.created_at())
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(video.clone()),
            Err(err) => {
                println!("{:#?}", err);
                Err("Error inserting video".to_string())
            }
        };
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Video<'a>, String> {
        return match sqlx::query(
            r#"
                SELECT vd.id, vd.resource_id, vd.file_path, vd.created_at
                FROM videos vd
                LEFT JOIN jobs jb ON jb.video_id = vd.id
                WHERE id = $1
            "#,
        )
        .bind(id)
        .map(|row: PgRow| {
            Video::new(
                row.get::<Uuid, &str>("id").to_string(),
                row.get::<Uuid, &str>("resource_id").to_string(),
                row.get::<String, &str>("file_path"),
                row.get::<NaiveDateTime, &str>("created_at"),
            )
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(video) => Ok(video),
            Err(err) => {
                println!("{:#?}", err);
                Err("Error finding video".to_string())
            }
        };
    }
}
