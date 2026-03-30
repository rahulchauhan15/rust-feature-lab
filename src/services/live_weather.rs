use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;

use super::weather_service::{WeatherResponse, WeatherService};

#[derive(serde::Deserialize)]
struct OwmResponse {
    name: String,
    main: OwmMain,
    weather: Vec<OwmWeather>,
    wind: OwmWind,
}

#[derive(serde::Deserialize)]
struct OwmMain {
    temp: f64,
    humidity: u8,
}

#[derive(serde::Deserialize)]
struct OwmWeather {
    description: String,
}

#[derive(serde::Deserialize)]
struct OwmWind {
    speed: f64,
}

pub struct LiveWeatherService {
    api_key: String,
    client: reqwest::Client,
}

impl LiveWeatherService {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl WeatherService for LiveWeatherService {
    async fn get_weather(&self, city: &str) -> Result<WeatherResponse> {
        tracing::debug!(city, "LiveWeatherService: calling OpenWeatherMap");

        let raw: OwmResponse = self
            .client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[
                ("q",     city),
                ("appid", self.api_key.as_str()),
                ("units", "metric"),
            ])
            .send()
            .await
            .context("failed to reach OpenWeatherMap")?
            .error_for_status()
            .context("OpenWeatherMap returned a non-2xx status")?
            .json::<OwmResponse>()
            .await
            .context("failed to deserialise OpenWeatherMap response")?;

        let description = raw
            .weather
            .into_iter()
            .next()
            .map(|w| w.description)
            .ok_or_else(|| anyhow!("OpenWeatherMap returned no weather entries for '{city}'"))?;

        Ok(WeatherResponse {
            city:                raw.name,
            temperature_celsius: raw.main.temp,
            description,
            humidity_percent:    raw.main.humidity,
            // OWM reports wind in m/s; convert and round to one decimal.
            wind_speed_kmh: (raw.wind.speed * 3.6 * 10.0).round() / 10.0,
        })
    }
}
