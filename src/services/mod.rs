pub mod weather_service;
pub mod s3_service;

mod live_weather;
#[cfg(feature = "mock")]
mod mock_weather;

use std::sync::Arc;
use weather_service::WeatherService;

pub fn build_weather_service(api_key: &str) -> Arc<dyn WeatherService> {
    if cfg!(feature = "mock") && std::env::var_os("MOCK_WEATHER").is_some() {
        tracing::warn!("Weather -> MOCK mode via env");
        #[cfg(feature = "mock")]
        {
            return Arc::new(mock_weather::MockWeatherService);
        }
    }

    tracing::info!("LiveWeatherService initialised");
    Arc::new(live_weather::LiveWeatherService::new(api_key))
}
