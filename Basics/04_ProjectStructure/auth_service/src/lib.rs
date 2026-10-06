#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = rand::random_range(100..500);

    println!("The timeout is: {timeout}");

    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
