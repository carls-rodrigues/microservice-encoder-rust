#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use rust_video_encoder_service::domain::Job;
    use rust_video_encoder_service::domain::Video;

    #[test]
    fn test_new_job() {
        let video = Video::new(
            Uuid::new_v4().to_string(),
            Uuid::new_v4().to_string(),
            "any_path".to_string(),
            chrono::Utc::now().naive_utc(),
        );

        let job = Job::new("any_output".to_string(), "any_status".to_string(), &video).unwrap();

        assert!(job.validate(&job).is_ok())
    }
}
