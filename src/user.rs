use reqwest::blocking::Client;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};

pub fn fetch_user() {
    const API_ENDPOINT: &str = "https://random-data-api.com/api/v2/users";
    let client: Client = Client::new();
    let response = client.get(API_ENDPOINT).send();
    let mut details: HashMap<String, String> = HashMap::new();

    #[derive(Debug, Deserialize)]
    struct Person {
        uuid: String,
        name: String,
        surname: String,
        email: String,
    }

    match response {
        Ok(res) => {
            println!("Status: {}", res.status());

            let response_as_text: String = res.text().expect("Failed to read response body");

            println!("{}", response_as_text);
            let person: Result<Person> = serde_json::from_str(&response_as_text);
            match person {
                Ok(person) => {
                    println!("UUID: {}", person.uuid);
                    println!("Name: {}", person.name);
                    println!("Surname: {}", person.surname);
                    println!("Email: {}", person.email);
                }
                Err(err) => {
                    eprintln!("Failed to deserialize JSON: {:?}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to send request: {:?}", err);
        }
    }
}