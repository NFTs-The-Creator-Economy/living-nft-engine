use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub weather: Vec<WeatherCondition>,
    pub main: MainWeather,
    pub wind: Wind,
    pub dt: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherCondition {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MockApiResponse {
    pub temperature: f64,
    pub humidity: u32,
    pub wind_speed: f64,
    pub weather_condition: String,
    pub timestamp: i64,
    pub location: String,
}

pub struct WeatherClient {
    client: reqwest::Client,
    api_url: String,
    api_key: Option<String>,
}

impl WeatherClient {
    pub fn new(api_url: String, api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url,
            api_key,
        }
    }

    pub async fn fetch_weather(&self, location: &str) -> Result<WeatherData> {
        let url = if let Some(ref key) = self.api_key {
            format!("{}?q={}&appid={}&units=metric", self.api_url, location, key)
        } else {
            format!("{}?q={}&units=metric", self.api_url, location)
        };

        let response = self.client.get(&url).send().await?;
        let weather_data: WeatherData = response.json().await?;
        
        Ok(weather_data)
    }

    pub async fn fetch_mock_weather(&self, location: &str) -> Result<MockApiResponse> {
        // Generate mock data based on location name hash for consistency
        let hash = location.chars().map(|c| c as u32).sum::<u32>();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let mock_data = MockApiResponse {
            temperature: 15.0 + (hash % 30) as f64, // 15-45°C
            humidity: 30 + (hash % 60), // 30-90%
            wind_speed: 5.0 + (hash % 25) as f64, // 5-30 m/s
            weather_condition: self.get_weather_condition_from_hash(hash),
            timestamp,
            location: location.to_string(),
        };

        Ok(mock_data)
    }

    fn get_weather_condition_from_hash(&self, hash: u32) -> String {
        match hash % 6 {
            0 => "clear".to_string(),
            1 => "clouds".to_string(),
            2 => "rain".to_string(),
            3 => "snow".to_string(),
            4 => "thunderstorm".to_string(),
            _ => "mist".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_weather() {
        let client = WeatherClient::new(
            "https://api.openweathermap.org/data/2.5/weather".to_string(),
            None,
        );

        let weather = client.fetch_mock_weather("San Francisco").await.unwrap();
        assert!(weather.temperature > 0.0);
        assert!(weather.humidity > 0);
        assert!(weather.wind_speed > 0.0);
        assert!(!weather.weather_condition.is_empty());
    }
}
