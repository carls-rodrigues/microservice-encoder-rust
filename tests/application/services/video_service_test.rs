use uuid::Uuid;

use rust_video_encoder_service::application::repositories::{
    VideoRepository, VideoRepositoryImplementation,
};
use rust_video_encoder_service::domain::Video;
use rust_video_encoder_service::framework::database::db::Database;

async fn prepare<'a>() -> Result<(Video<'a>, VideoRepositoryImplementation), String> {
    let db = Database::new_db_test().await;
    let video = Video::new(
        Uuid::new_v4().to_string(),
        Uuid::new_v4().to_string(),
        "video.mp4".to_string(),
        chrono::Utc::now().naive_utc(),
    );

    let repository_db = VideoRepositoryImplementation::instance(&db);
    let _ = repository_db.insert(&video.clone()).await.unwrap();
    return Ok((video.clone(), repository_db));
}

#[cfg(test)]
pub mod tests {
    use std::env;

    use rust_video_encoder_service::application::services::VideoService;

    use crate::application::services::video_service_test::prepare;

    #[tokio::test]
    async fn test_video_service_download() {
        let (video, video_repository) = prepare().await.unwrap();
        let bucket_name =
            env::var("GOOGLE_CLOUD_STORAGE_BUCKET").expect("Failed to get bucket name");
        let video_service = VideoService::new(&video, video_repository);
        let download = video_service.download(&bucket_name).await;
        assert!(download.is_ok());
        let fragment = video_service.fragment().await;
        assert!(fragment.is_ok());
        let encode = video_service.encode().await;
        assert!(encode.is_ok());
        let finish = video_service.finish().await;
        assert!(finish.is_ok());
    }
}
