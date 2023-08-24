use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{Job, Video};

#[async_trait]
pub trait JobRepository<'a> {
    async fn insert(&self, job: &'a Job<'a>) -> Result<Job<'a>, String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Job<'a>, String>;
    async fn update(&self, job: &'a Job<'a>) -> Result<Job<'a>, String>;
}

pub struct JobRepositoryImplementation {
    connection: PgPool,
}

impl JobRepositoryImplementation {
    pub fn instance(connection: &PgPool) -> Self {
        Self {
            connection: connection.clone(),
        }
    }
}

// id: String,
// output_bucket_path: String,
// status: String,
// video: &'a Video<'a>,
// video_id: String,
// error: Option<String>,
// created_at: NaiveDateTime,
// updated_at: NaiveDateTime,
#[async_trait]
impl<'a> JobRepository<'a> for JobRepositoryImplementation {
    async fn insert(&self, job: &'a Job<'a>) -> Result<Job<'a>, String> {
        return match sqlx::query(
            r#"
                INSERT INTO jobs (job_id, output_bucket_path, status, video_id, error, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
            .bind(&job.id())
            .bind(&job.output_bucket_path())
            .bind(&job.status())
            .bind(&job.video_id())
            .bind(&job.error())
            .bind(&job.created_at())
            .bind(&job.updated_at())
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(job.clone()),
            Err(err) => {
                println!("INSERT JOB {:#?}", err);
                Err("Error inserting job".to_string())
            }
        };
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Job<'a>, String> {
        return match sqlx::query(
            r#"
                SELECT
                    jb.id,
                    jb.output_bucket_path,
                    jb.status,
                    jb.video_id,
                    jb.error,
                    jb.created_at,
                    jb.updated_at
                    vd.id,
                    vd.resource_id,
                    vd.file_path,
                    vd.created_at
                FROM jobs jb
                LEFT JOIN videos vd
                ON jb.video_id = vd.id
                WHERE id = $1
            "#,
        )
        .bind(id)
        .map(|row| {
            let video = Video::new(
                row.get::<Uuid, &str>("vd.id").to_string(),
                row.get::<Uuid, &str>("vd.resource_id").to_string(),
                row.get::<String, &str>("vd.file_path"),
                row.get::<NaiveDateTime, &str>("vd.created_at"),
            );
            let video_ref: &'a Video = Box::leak(Box::new(video));
            Job::from(
                row.get::<Uuid, &str>("jb.id").to_string(),
                row.get::<String, &str>("jb.output_bucket_path"),
                row.get::<String, &str>("status"),
                video_ref,
                row.get::<Uuid, &str>("jb.video_id").to_string(),
                row.get::<Option<String>, &str>("jb.error"),
                row.get::<NaiveDateTime, &str>("jb.created_at"),
                row.get::<NaiveDateTime, &str>("jb.updated_at"),
            )
            .unwrap()
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(job) => Ok(job),
            Err(err) => {
                println!("SELECT JOB {:#?}", err);
                Err("Error finding job".to_string())
            }
        };
    }

    async fn update(&self, _job: &'a Job<'a>) -> Result<Job<'a>, String> {
        todo!()
        // return match sqlx::query(
        //     r#"
        //         UPDATE jobs SET
        //             output_bucket_path = $1,
        //             status = $2,
        //             video_id = $3,
        //             error = $4,
        //             created_at = $5,
        //             updated_at = $6
        //         WHERE id = $7
        //     "#
        // )
        //     .bind(&job.output_bucket_path())
    }
}
