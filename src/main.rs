extern crate chrono;
extern crate sun_times;
extern crate config;

use std::io;
use std::process::Command;
use std::thread;

use chrono::prelude::*;
use chrono::{ Utc, DateTime, Local };

fn main() {
    let (lat, long) = read_configuration();
    run_process_loop(lat, long);
}

// Returns the sunset and sunrise time for the current date.
fn get_sunrise_and_sunset(lat: f64, long: f64, altitude: f64) -> (DateTime<Utc>, DateTime<Utc>) {
    let current_date = Utc.from_local_date(&Local::today().naive_local()).unwrap();
    sun_times::sun_times(current_date, lat, long, altitude)
}

// Sets the system to use dark mode by executing an apple script
fn set_dark_mode() {
    Command::new("osascript")
            .args(&["-e", "tell app \"System Events\" to tell appearance preferences to set dark mode to true"])
            .output()
            .expect("");
}

// Sets the system to use light mode by executing an apple script
fn set_light_mode() {
    Command::new("osascript")
            .args(&["-e", "tell app \"System Events\" to tell appearance preferences to set dark mode to false"])
            .output()
            .expect("");
}

// Runs a loop to check the sunset/sunrise time from the current time and change the theme based on the result.
fn run_process_loop(lat: f64, long: f64) {
    // Get the sunset and sunrise times.
    let mut times = get_sunrise_and_sunset(lat, long, 8.0);
    
    loop {
        let current_time = Local::now();
        let sunrise = Local.from_utc_datetime(&times.0.naive_utc());
        let sunset = Local.from_utc_datetime(&times.1.naive_utc());
        
        // Set the theme to light mode
        if current_time.gt(&sunrise) && current_time.lt(&sunset) {
            set_light_mode();
        }

        // // Set the theme to dark mode.
        if current_time.gt(&sunrise) && current_time.gt(&sunset) {
            set_dark_mode();
        }

        times = get_sunrise_and_sunset(28.817631, -81.303352, 8.0);
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn read_configuration() -> (f64, f64) {
    let mut location = config::Config::new();
    location.merge(config::File::with_name("Location")).unwrap();

    let lat = match location.get("lat") {
        Ok(lat) => {
            lat
        },
        Err(_) => {
            println!("Your latitude is not set, please set it to get the proper sunrise and sunset time");
            print!("Latitude: ");
            let mut lat = String::new();

            io::stdin()
                .read_line(&mut lat)
                .expect("Failed to get latitude");

            let trimmed = lat.trim();
            let lat = trimmed.parse::<f64>().expect("Expected a float");

            lat
        }
    };

    let long = match location.get("long") {
        Ok(long) => {
            long
        },
        Err(_) => {
            println!("Your longitude is not set, please set it to get the proper sunrise and sunset time");
            print!("Longitude: ");
            let mut lat = String::new();

            io::stdin()
                .read_line(&mut lat)
                .expect("Failed to get longitude");

            let trimmed = lat.trim();
            let long = trimmed.parse::<f64>().expect("Expected a float");
            
            long
        }
    };

    (lat, long)
}