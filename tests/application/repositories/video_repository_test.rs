#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use rust_video_encoder_service::application::repositories::{
        VideoRepository, VideoRepositoryImplementation,
    };
    use rust_video_encoder_service::domain::Video;
    use rust_video_encoder_service::framework::database::db::Database;

    #[tokio::test]
    async fn test_video_repository_insert() {
        let db = Database::new_db_test().await;
        let video = Video::new(
            Uuid::new_v4().to_string(),
            Uuid::new_v4().to_string(),
            "any_path".to_string(),
            chrono::Utc::now().naive_utc(),
        );

        let repository_db = VideoRepositoryImplementation::instance(&db);
        let insert_video = repository_db.insert(&video).await.unwrap();
        assert_eq!(insert_video.id(), video.id());

        let get_video = repository_db.find_by_id(&video.id()).await.unwrap();
        assert_eq!(get_video.id(), video.id());
    }
}
