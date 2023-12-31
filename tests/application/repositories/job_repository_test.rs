#[cfg(test)]
mod tests {
    use chrono::Utc;
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
        )
        .unwrap();

        let repository_db = VideoRepositoryImplementation::instance(&db);
        let _ = repository_db.insert(&video).await;
        let job_repository = JobRepositoryImplementation::instance(&db);
        job_repository.insert(&job.clone()).await.unwrap();
        let get_job = job_repository.find_by_id(&job.clone().id()).await.unwrap();
        assert_eq!(get_job.id(), job.clone().id());
        assert_eq!(get_job.video_id(), job.clone().video_id());
    }

    #[tokio::test]
    async fn test_job_repository_update() {
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
        )
        .unwrap();

        let repository_db = VideoRepositoryImplementation::instance(&db);
        let _ = repository_db.insert(&video).await;
        let job_repository = JobRepositoryImplementation::instance(&db);
        job_repository.insert(&job.clone()).await.unwrap();
        let get_job = job_repository.find_by_id(&job.clone().id()).await.unwrap();
        let update_job = Job::from(
            get_job.id().to_string(),
            get_job.output_bucket_path().to_string(),
            "Complete".to_string(),
            get_job.video(),
            get_job.video_id().to_string(),
            None,
            get_job.created_at(),
            Utc::now().naive_utc(),
        )
        .unwrap();
        let job_update = job_repository.update(&update_job).await.unwrap();
        let get_job = job_repository
            .find_by_id(&job_update.clone().id())
            .await
            .unwrap();
        assert_eq!(get_job.id(), job_update.clone().id());
    }
}
