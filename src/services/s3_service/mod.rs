use std::sync::Arc;

use crate::services::s3_service::s3_service::S3Service;

pub mod s3_service;

mod live_s3;
#[cfg(feature = "mock")]
mod mock_s3;


pub fn build_s3_service(bucket: &str) -> Arc<dyn S3Service> {
    if cfg!(feature = "mock") && std::env::var_os("MOCK_S3").is_some() {
        tracing::warn!("S3 -> MOCK mode via env");
        #[cfg(feature = "mock")]
        {
            return Arc::new(mock_s3::MockS3Service);
        }
    }

    tracing::info!("LiveS3Service initialised");
    Arc::new(live_s3::LiveS3Service::new(bucket))
}