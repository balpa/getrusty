use reqwest::blocking::Client;

pub fn fetch_user() {
    let client = Client::new();

    let response = client.get("https://random-data-api.com/api/v2/users").send();

    match response {
        Ok(res) => {
            println!("Status: {}", res.status());

            let body = res.text().expect("Failed to read response body");

            println!("Body: {}", body);
        }
        Err(err) => {
            eprintln!("Failed to send request: {}", err);
        }
    }
}