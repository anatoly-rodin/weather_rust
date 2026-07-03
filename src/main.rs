#![allow(unused_imports, dead_code)]

use std::{env, io};
use serde::Deserialize;
use colored::*;
use reqwest::blocking::Response;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let api_key: String = env::var("API_KEY").expect("API_KEY param is missed in .env file.");
}

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
    temp: f64,
    humidity: f64,
    pressure: f64,
}

/// Struct to represent the wind information.
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

/// Function to get the weather information from OpenWeatherMap API.
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key,
    );

    let response: Response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json()?;

    Ok(response_json)
}

/// Function to display the weather information into terminal output.
fn display_weather_info(response: &WeatherResponse) {
    let description: &str = match response.weather.get(0) {
        Some(weather) => &weather.description,
        _ => "no desc",
    };
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_output_text: String = format!(
        "Weather in {}: {}
        > Temperature: {:.1} C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    println!("{}", weather_output_text);
}