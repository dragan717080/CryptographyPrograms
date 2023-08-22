extern crate serde;
extern crate serde_json;

use chrono::{DateTime, Datelike, Duration, Local, Utc, TimeZone};
use std::time::Duration as StdDuration;
use humantime::{format_duration, FormattedDuration};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json::{Value, from_str};
use std::error::Error as StdError;

use std::fs::File;
use std::io::{self, Read, Write};

use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct Data<T> {
    value: T,
}

fn main() { 
    let s = get_random_date_in_past(18, 100);
    println!("{}", s);
}

pub fn get_random_name() -> String {
    let mut random_name = get_random_string_from_list(read_string_list_from_json("data/names.json").unwrap());
    let random_surname = get_random_string_from_list(read_string_list_from_json("data/surnames.json").unwrap());
    random_name = random_name + format!(" {}", random_surname).as_str();
    random_name
}

pub fn read_string_list_from_json(file_name: &str) -> Result<Vec<String>, Box<dyn StdError>> {
    let mut file = File::open(file_name)?;
    let mut json_content = String::new();

    // Handle std::io::Error separately (file doesn't exist or can't be read)
    if let Err(io_error) = file.read_to_string(&mut json_content) {
        return Err(Box::new(io_error));
    }

    // Handle serde_json::Error separately
    let names: Vec<String> = match serde_json::from_str(&json_content) {
        Ok(names) => names,
        Err(json_error) => return Err(Box::new(json_error)),
    };

    Ok(names.clone())
}

pub fn read_json_file<T: DeserializeOwned>(filename: &str) -> Result<T, Box<dyn StdError>> {
    let mut file = File::open(filename)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let deserialized: T = serde_json::from_str(&json_string)?;

    Ok(deserialized)
}

pub fn get_random_string_from_list(list_str: Vec<String>) -> String {
    list_str[rand::thread_rng().gen_range(0..list_str.len())].clone()
}

pub fn get_random_email(name: &str) -> String {
    let email_provider = get_random_string_from_list(read_string_list_from_json("data/email_providers.json").unwrap());
    let random_number: i32 = rand::thread_rng().gen_range(0..1000);
    let mut modified_name = name.replace(" ", "").to_lowercase();
    format!("{}{}@{}.com", modified_name, random_number, email_provider)
}

pub fn get_random_phone() -> u32 {
    let mut phone_str = String::new();
    for i in 0..7 {
        let start_number = if i == 0 { 1 } else { 0 };
        phone_str.push_str(&(rand::thread_rng().gen_range(start_number..=9).to_string()));
    }
    phone_str.parse().unwrap()
}

// Get random date in past from x to y years ago
pub fn get_random_date_in_past(x: u32, y: u32) -> DateTime<Utc> {
    let current_utc_time = Utc::now();
    let min_duration = Duration::days(365 * (y.min(x)) as i64);
    let max_duration = Duration::days(365 * (y.max(x)) as i64);
    let random_duration = Duration::days(rand::thread_rng().gen_range(min_duration.num_days()..=max_duration.num_days()));
    current_utc_time - random_duration
}

pub fn write_to_json_file<T: Serialize>(data: T, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(&data)?;

    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
