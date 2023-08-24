#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use rust_video_encoder_service::application::repositories::{
        JobRepository, JobRepositoryImplementation, VideoRepository, VideoRepositoryImplementation,
    };
    use rust_video_encoder_service::domain::{Job, Video};
    use rust_video_encoder_service::framework::database::db::Database;

    #[tokio::test]
    async fn test_job_repository_insert() {
        let db = Database::new_db_test().await;
        let video = Video::new(
            Uuid::new_v4().to_string(),
            Uuid::new_v4().to_string(),
            "any_path".to_string(),
            chrono::Utc::now().naive_utc(),
        );
        let job = Job::new(
            "any_path".to_string(),
            "Pending".to_string(),
            &video,
            video.id().to_string(),
        );

        let repository_db = VideoRepositoryImplementation::instance(&db);
        let _insert_video = repository_db.insert(&video).await;
        let job_repository = JobRepositoryImplementation::instance(&db);
        job_repository.insert(&job.unwrap()).await.unwrap();
    }
}
