use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "YOUR_API_KEY"; // Replace with your OpenWeatherMap API key
    let city = "London"; // Change this to your city of choice

    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    let main = response["main"].as_object().unwrap();
    let temperature = main["temp"].as_f64().unwrap();
    let weather = response["weather"][0]["description"].as_str().unwrap();

    println!("Current temperature in {}: {:.1}°C, Weather: {}", city, temperature, weather);

    Ok(())
}