use reqwest;
use serde::{de::DeserializeOwned, Deserialize};
use std::{env, time::Duration};

// If this takes too long then bail out.
const TIMEOUT_SECONDS: u64 = 2;

fn main() {
    let output = match get_location().and_then(get_temperature) {
        Ok(temp) => temp,
        Err(e) => match e {
            // If we hit a timeout, print nothing
            Err::Timeout => "".to_string(),
            Err::Other(msg) => msg,
        },
    };

    print!("{}", output)
}

enum Err {
    Timeout,
    Other(String),
}

impl Err {
    fn message(&self, m: &str) -> Err {
        match self {
            Err::Timeout => Err::Timeout,
            Err::Other(msg) => Err::Other(format!("{}: {}", m, msg)),
        }
    }
}

fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, Err> {
    reqwest::blocking::Client::new()
        .get(url)
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .send()
        .and_then(|r| r.json::<T>())
        .map_err(|e| match e.is_timeout() {
            true => Err::Timeout,
            false => Err::Other(format!("Error making request: {}", e)),
        })
}

#[derive(Deserialize)]
struct IPInfroResponse {
    loc: String,
}

fn get_location() -> Result<String, Err> {
    fetch::<IPInfroResponse>("https://ipinfo.io")
        .map(|r| r.loc)
        .map_err(|e| e.message("Failed to fetch IP address"))
}

#[derive(Deserialize)]
struct PirateWeatherResponse {
    currently: PirateWeatherCurrently,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PirateWeatherCurrently {
    apparent_temperature: f32,
}

fn get_pirate_weather_api_key() -> Result<String, Err> {
    env::var("PIRATE_WEATHER_API_KEY")
        .map_err(|e| Err::Other(format!("Unable to find pirate weather API key: {}", e)))
}

fn get_temperature(location: String) -> Result<String, Err> {
    get_pirate_weather_api_key()
        .map(|api_key| {
            format!(
                "https://api.pirateweather.net/forecast/{}/{}",
                api_key, location
            )
        })
        .and_then(|url| fetch::<PirateWeatherResponse>(&url))
        .map(|r| format!("{}°", r.currently.apparent_temperature))
        .map_err(|e| e.message("Unable to fetch weather data"))
}
