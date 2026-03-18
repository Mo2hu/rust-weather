use std::fmt;

pub struct WeatherDisplay {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
    conditions: String,
}

impl WeatherDisplay {
    pub fn new(temperature: f64, humidity: f64, wind_speed: f64, conditions: String) -> Self {
        WeatherDisplay {
            temperature,
            humidity,
            wind_speed,
            conditions,
        }
    }

    pub fn display(&self) -> String {
        format!("\nWeather Report:\n------------------\nTemperature: {:.1}°C\nHumidity: {:.0}%\nWind Speed: {:.1} m/s\nConditions: {}\n", 
               self.temperature, self.humidity, self.wind_speed, self.conditions)
    }
}

impl fmt::Display for WeatherDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}