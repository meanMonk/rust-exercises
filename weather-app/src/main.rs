#![allow(dead_code)]
#![allow(unused_variables)]

use std::{env, io::{self, stdout}, process::exit};

use chrono::{DateTime, Local};
use crossterm::{execute, terminal::{Clear, ClearType}};
use dotenv::dotenv;
use error_chain::error_chain;
use serde::Deserialize;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Debug, Deserialize)]
struct ResponseMain {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u64,
    humidity: u64,
    sea_level: u64,
    grnd_level: u64,
}
#[derive(Debug, Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: u64,
}
#[derive(Debug, Deserialize)]
struct Weather {
    id: f64,
    main: String,
    description: String,
    icon: String,
}
#[derive(Debug, Deserialize)]
struct Sys {
    country: String,
    sunrise: i64,
    sunset: i64,
}
#[derive(Debug, Deserialize)]
struct Response {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: ResponseMain,
    visibility: u128,
    wind: Wind,
    dt: u128,
    sys: Sys,
    timezone: u64,
    id: u64,
    name: String,
}

// clear screen utility.
fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).expect("failed to clear the screen!");
}

async fn load_weather_data(city: &str) -> Result<Response> {
    let api_key = env::var("WEATHER_API_KEY").expect("Unable to find api key!");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );
    let res = reqwest::get(&url).await?;
    let data = res.json::<Response>().await?;
    Ok(data)
}

fn unix_to_local_time(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0).expect("invalid timestamp");

    let datetime: DateTime<Local> = DateTime::from(datetime);

    datetime.format("%d/%m/%Y %H:%M").to_string()
}

fn display_info(data: Response) {
    let main = data.main;
    let wind = data.wind;
    let sys = data.sys;
    let weather = &data.weather[0];

    println!("");
    println!(
        "🌍 Weather for City: {}, {} (Lat: {:.2}, Lon: {:.2})",
        data.name, sys.country, data.coord.lat, data.coord.lon
    );
    println!("------------------------------------------");
    // Convert UNIX timestamps to human-readable local time
    let sunrise_time = unix_to_local_time(sys.sunrise);
    let sunset_time = unix_to_local_time(sys.sunset);
    println!(
        "🌅  Sunrise: {} | 🌇  Sunset: {}",
        sunrise_time, sunset_time
    );
    println!(
        "🌡️  Temperature: {:.2} K (Feels like {:.2} K)",
        main.temp, main.feels_like
    );
    println!(
        "📉 Min Temp: {:.2} K   |   📈 Max Temp: {:.2} K",
        main.temp_min, main.temp_max
    );
    println!("💧 Humidity: {}%", main.humidity);
    println!("🌬️  Wind Speed: {:.2} m/s from {}°", wind.speed, wind.deg);
    println!("☁️  Cloudiness: ({})", weather.description);
    println!("👁️  Visibility: {} m", data.visibility);
    println!("------------------------------------------");
    println!("");
}

fn print_title() {
    println!("{:-^40}", " ");
    println!("🎉 Welcome to Weather App 🎉");
    println!("{:-^40}", " ");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    print_title();
    loop {
        println!("Please enter city name : ");
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Unable to read city name!");
        let city = city.trim();

        match load_weather_data(city).await {
            Ok(data) => display_info(data),
            Err(err) => println!("unable to fetch data {:?}", err),
        };

        println!("Do you want to check for another city? (yes/no)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("unable to read answer");
        
        if "yes" == answer.trim().to_lowercase() {
            clear_screen();
            print!("\x1Bc");
            print_title();
        } else {
            println!("");
            println!("{:=^40}", " 💚 thank you 💚 ");
            exit(0);
        }
        
    }
}
