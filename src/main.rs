extern crate chrono;
extern crate sun_times;

use std::env;
use std::process::Command;
use std::thread::sleep;

use chrono::TimeZone;
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
fn set_theme(dark: bool) {
    let arg: &str = match dark {
        true => "tell app \"System Events\" to tell appearance preferences to set dark mode to true",
        false => "tell app \"System Events\" to tell appearance preferences to set dark mode to false"
    };

    Command::new("osascript")
        .args(&["-e", arg])
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
            set_theme(false);
        }

        // Set the theme to dark mode.
        if current_time.gt(&sunrise) && current_time.gt(&sunset) {
            set_theme(true);
        }

        times = get_sunrise_and_sunset(lat, long, 8.0);
        sleep(std::time::Duration::from_secs(60 * 10));
    }
}

fn read_configuration() -> (f64, f64) {
    let args: Vec<String> = env::args().collect();

    let lat: f64 = match &args[1].parse() {
        Ok(num) => { *num },
        Err(_) => { panic!("Error: First argument(lat) should be a float"); },
    };

    let long: f64 = match &args[2].parse() {
        Ok(num) => { *num },
        Err(_) => { panic!("Error: Second argument(long) should be a float"); },
    };

    (lat, long)
}