use std::io;
use serde::Deserialize;
use colored::*;

fn main() {}

/// Struct to deserialize the JSON response from openWeatherApp API.
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

/// Struct to represent the weather description.
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

/// Struct to represent the main weather parameters.
#[derive(Deserialize, Debug)]
struct Main {
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

/// Struct to represent the wind information.
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

/// Function to get the weather information from OpenWeatherMap API.
fn get_weather(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key,
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;

    Ok(response_json)
}