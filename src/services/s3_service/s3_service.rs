use axum::async_trait;

#[async_trait]
pub trait S3Service: Send + Sync + 'static {
    async fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<String>;
}
