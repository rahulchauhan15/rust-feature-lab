use crate::services::s3_service::s3_service::S3Service;

pub struct MockS3Service;

#[async_trait::async_trait]
impl S3Service for MockS3Service {
    async fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<String> {
        tracing::debug!(key = %key, size = data.len(), "MockS3Service: upload simulated");
        Ok(format!("mock://bucket/{key}"))
    }
}


