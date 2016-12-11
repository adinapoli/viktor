
extern crate rustc_serialize;
extern crate hyper;

pub use hyper::client::Client;
use std::io::Read;
use std;
use std::env;
use rustc_serialize::json;

static APIXU_URL: &'static str = "https://api.apixu.com/v1/";

struct ApixuCfg {
    api_key: String,
}

#[derive(RustcDecodable, Debug, Default)]
pub struct CurrentWeather {
    /// Local time when the real time data was updated.
    last_updated: Option<String>,
    /// Local time when the real time data was updated in unix time.
    last_updated_epoch: Option<i32>,
    /// Temperature in celsius
    temp_c: f32,
    /// Temperature in fahrenheit
    temp_f: f32,
    // condition:text	string	Weather condition text
    // condition:icon	string	Weather icon url
    // condition:code	int	Weather condition unique code.
    /// Wind speed in miles per hour
    wind_mph: f32,
    /// Wind speed in kilometer per hour
    wind_kph: f32,
    /// Wind direction in degrees
    wind_degree: Option<i32>,
    /// Wind direction as 16 point compass. e.g.: NSW
    wind_dir: Option<String>,
    /// Pressure in millibars
    pressure_mb: f32,
    /// Pressure in inches
    pressure_in: f32,
    /// Precipitation amount in millimeters
    precip_mm: f32,
    /// Precipitation amount in inches
    precip_in: f32,
    /// Humidity as percentage
    humidity: Option<u8>,
    /// Cloud cover as percentage
    cloud: Option<u8>,
    /// Feels like temperature as celcius
    feelslike_c: f32,
    /// Feels like temperature as fahrenheit
    feelslike_f: f32,
    is_day: Option<u8>, // 1 = Yes 0 = No
}

#[derive(Debug)]
pub enum ApixuError {
    FailedToContactRemoteHost(hyper::error::Error),
    InvalidRequest(String, hyper::client::Response),
    IOError(std::io::Error),
    ParseJsonError(json::DecoderError),
}

impl From<hyper::error::Error> for ApixuError {
    fn from(err: hyper::error::Error) -> ApixuError {
        ApixuError::FailedToContactRemoteHost(err)
    }
}

impl From<json::DecoderError> for ApixuError {
    fn from(err: json::DecoderError) -> ApixuError {
        ApixuError::ParseJsonError(err)
    }
}

impl From<std::io::Error> for ApixuError {
    fn from(err: std::io::Error) -> ApixuError {
        ApixuError::IOError(err)
    }
}

lazy_static! {
    static ref APIXU_CFG: ApixuCfg = {
        let acfg = ApixuCfg {
            api_key: env::var("APIXU_API_KEY").unwrap_or(String::from(""))
        };
        acfg
    };
}

fn mk_url(uri_path: &str) -> String {
    String::from(format!("{}{}?key={}", APIXU_URL, uri_path, APIXU_CFG.api_key))
}

/// Gets the current weather based on Auto IP.
// TODO: Better error handling.
pub fn current_weather(client: &hyper::client::Client) -> Result<CurrentWeather, ApixuError> {
    let url = mk_url("current.json");
    let mut response = try!(client.get(&url).send());
    if response.status != hyper::status::StatusCode::Ok {
        return Err(ApixuError::InvalidRequest(url, response));
    }
    let mut body = String::new();
    let _ = try!(response.read_to_string(&mut body));
    let cw: CurrentWeather = try!(json::decode(&body));
    Ok(cw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decode_a_current_weather_request() {
        let client = Client::new();
        match current_weather(&client) {
            Ok(_) => assert!(true, true),
            Err(e) => panic!(format!("{:?}", e)),
        }
    }
}
