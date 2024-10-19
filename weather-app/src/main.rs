

/* 
🌩 
🌧 
🌨 
🌪 
☁ 
🌥 
🌥 
🌦 
*/

use std::io;

use chrono::{DateTime, Local};

fn display_info(date:  &DateTime<Local>) {
    println!("=======================");
    println!("🌥");
    println!("Date : {}", date.format("%d/%m/%y"));
    println!("Summary : {} ", "clear");
    println!("Temprature : {} ˚c",9);
    println!("Wind speed : {} km/h",10);
    println!("Wind Dir : {} ","nw");
    println!("=======================");
}

fn main() {
    println!("🌥 Weather App");
    println!("Enter city name ?");
    let mut city = String::new();
    
    io::stdin()
    .read_line(&mut city)
    .expect("Unable to read the name");

    let city = city.trim().to_string();
    
    println!("Wether for city: {}", city.to_uppercase());
    let date = Local::now();
    display_info(&date);
}