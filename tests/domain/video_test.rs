#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use rust_video_encoder_service::domain::Video;

    #[test]
    fn test_validate_if_video_is_empty() {
        let video = Video::new(
            String::from(""),
            String::from(""),
            String::from(""),
            Utc::now().naive_utc(),
        );
        assert!(video.validate().is_err());
    }

    #[test]
    fn test_validate_if_id_is_not_uuid() {
        let video = Video::new(
            String::from("invalid-uuid"),
            Uuid::new_v4().to_string(),
            String::from("any_path"),
            Utc::now().naive_utc(),
        );
        assert!(video.validate().is_err());
        assert_eq!(video.validate().unwrap_err(), "Id is empty or invalid");
    }

    #[test]
    fn test_validate_if_resource_id_is_not_uuid() {
        let video = Video::new(
            Uuid::new_v4().to_string(),
            String::from("invalid-resource_uuid"),
            String::from("any_path"),
            Utc::now().naive_utc(),
        );
        assert!(video.validate().is_err());
        assert_eq!(
            video.validate().unwrap_err(),
            "Resource id is empty or invalid"
        );
    }

    #[test]
    fn test_video_validation() {
        let video = Video::new(
            Uuid::new_v4().to_string(),
            Uuid::new_v4().to_string(),
            String::from("any_path"),
            Utc::now().naive_utc(),
        );
        assert!(video.validate().is_ok());
    }
}
