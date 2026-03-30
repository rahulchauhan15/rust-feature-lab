use crate::services::s3_service::s3_service::S3Service;

pub struct LiveS3Service {
    bucket: String,
}

impl LiveS3Service {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self { bucket: bucket.into() }
    }
}

#[async_trait::async_trait]
impl S3Service for LiveS3Service {
    async fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<String> {
        tracing::info!(bucket = %self.bucket, key = %key, size = data.len(), "LiveS3Service: upload request received");

        // Placeholder: real S3 code should go here.
        let object_url = format!("s3://{}/{}", self.bucket, key);
        Ok(object_url)
    }
}
