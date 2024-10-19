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

fn display_info(date: &DateTime<Local>) {
    println!("=======================");
    println!("🌥");
    println!("Date : {}", date.format("%d/%m/%y"));
    println!("Summary : {} ", "clear");
    println!("Temprature : {} ˚c", 9);
    println!("Wind speed : {} km/h", 10);
    println!("Wind Dir : {} ", "nw");
    println!("=======================");
}

fn main() {
    println!("");
    println!("");
    println!("{:^40}", "🌥 Weather App 🎉 ");
    println!("{:-^40}", "-");
    println!("");
    loop {
        println!("Enter city name ?");
        let mut city = String::new();

        io::stdin()
            .read_line(&mut city)
            .expect("Unable to read the name");

        let city = city.trim().to_string();

        println!("Weather for city: {}", city.to_uppercase());
        let date = Local::now();
        
        display_info(&date);
        
        // ask for new input
        println!("Do you check for other city? (yes/no)");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Unable to read the name");
        
        if answer.trim() != "yes" {
            println!("");
            println!("{:-^40}", "-");
            println!("{:^40}", " 💚 thank you 💚 ");
            break;
        }
    }
}
