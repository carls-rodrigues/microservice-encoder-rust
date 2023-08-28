use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use google_cloud_storage::client::google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::google_cloud_auth::error::Error;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;

use crate::application::repositories::VideoRepository;
use crate::domain::Video;

pub struct VideoService<'a> {
    video: &'a Video<'a>,
    video_repository: Box<dyn VideoRepository<'a>>,
}

impl<'a> VideoService<'a> {
    pub fn new(video: &'a Video<'a>, video_repository: impl VideoRepository<'a> + 'static) -> Self {
        Self {
            video,
            video_repository: Box::new(video_repository),
        }
    }
    pub async fn download(&self, bucket_name: &str) -> Result<(), Error> {
        let client = self.client().await?;
        let get_object_request = GetObjectRequest {
            bucket: bucket_name.to_owned(),
            object: self.video.file_path().to_owned(),
            ..GetObjectRequest::default()
        };
        let download = client
            .download_object(&get_object_request, &Range::default())
            .await
            .expect("Failed to download file");
        let bytes = download.as_slice();
        let pathname = env::var("LOCAL_STORAGE_PATH").unwrap_or("tmp".to_string());
        let dir_path = Path::new(&pathname);
        let dir_exists = dir_path.exists();
        if !dir_exists {
            let _ = fs::create_dir(&pathname);
        }
        let _ = fs::create_dir(&pathname);
        let filename = format!("{}/{}{}", pathname, self.video.id(), ".mp4");
        let mut file = fs::File::create(&filename).expect("Unable to create file");
        file.write_all(&bytes).unwrap();
        Ok(())
    }
    pub async fn fragment(&self) -> Result<(), String> {
        let pathname = env::var("LOCAL_STORAGE_PATH").unwrap_or("tmp".to_string());
        let folder_name = format!("{}/{}", pathname, self.video.id());
        let _ = fs::create_dir(&folder_name).expect("Unable to create folder");
        let source = format!("{}/{}{}", pathname, self.video.id(), ".mp4");
        let target = format!("{}/{}{}", pathname, self.video.id(), ".frag");
        let cmd = Command::new("mp4fragment")
            .arg(source)
            .arg(target)
            .output()
            .expect("failed to execute process");

        self.output(cmd.stdout);
        Ok(())
    }
    pub async fn encode(&self) -> Result<(), String> {
        let pathname = env::var("LOCAL_STORAGE_PATH").unwrap_or("tmp".to_string());
        let mut cmd_args = vec![];
        let video_arg = format!("{}/{}{}", pathname, self.video.id(), ".frag");
        cmd_args.push(video_arg);
        cmd_args.push("--use-segment-timeline".to_string());
        cmd_args.push("-o".to_string());
        let output_arg = format!("{}/{}", pathname, self.video.id());
        cmd_args.push(output_arg);
        cmd_args.push("-f".to_string());
        cmd_args.push("--exec-dir".to_string());
        cmd_args.push("/opt/bento4/bin".to_string());
        let cmd = Command::new("mp4dash")
            .args(cmd_args)
            .output()
            .expect("failed to execute process");
        self.output(cmd.stdout);
        Ok(())
    }
    pub async fn finish(&self) -> Result<(), String> {
        let pathname = env::var("LOCAL_STORAGE_PATH").unwrap_or("tmp".to_string());
        let folder_name = format!("{}/{}", pathname, self.video.id());
        let _ = fs::remove_dir_all(&folder_name);
        let _ = fs::remove_file(format!("{}/{}{}", pathname, self.video.id(), ".mp4"));
        let _ = fs::remove_file(format!("{}/{}{}", pathname, self.video.id(), ".frag"));
        Ok(())
    }
    async fn client(&self) -> Result<Client, Error> {
        let credentials =
            CredentialsFile::new_from_file("/app/config/bucket-credential.json".to_owned()).await?;
        let config = ClientConfig::default()
            .with_credentials(credentials)
            .await?;
        let client = Client::new(config);
        Ok(client)
    }
    pub fn output(&self, out: Vec<u8>) {
        if out.len() > 0 {
            println!("Output: {}", String::from_utf8_lossy(&out));
        }
    }
}
