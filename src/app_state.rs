use std::sync::Arc;
use crate::services::{s3_service::s3_service::S3Service, weather_service::WeatherService};

#[derive(Clone)]
pub struct AppState {
    pub weather: Arc<dyn WeatherService>,
    pub s3:      Arc<dyn S3Service>,       
}
