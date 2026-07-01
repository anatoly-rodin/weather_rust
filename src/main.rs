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