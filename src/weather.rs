use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f32,
    pub description: String,
}

impl WeatherData {
    pub async fn fetch_weather(api_key: &str, city: &str) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
        let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

        let temperature = response["main"]["temp"].as_f64().unwrap() as f32;
        let description = response["weather"][0]["description"].as_str().unwrap_or("").to_string();

        Ok(WeatherData { temperature, description })
    }
}