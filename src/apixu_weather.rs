
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

type City = str;

#[derive(RustcDecodable, Debug, Default)]
pub struct Location {
    ///Latitude in decimal degree
    lat: f32,
    ///Longitude in decimal degree
    lon: f32,
    /// Location name
    name: String,
    /// Region or state of the location, if available
    region: Option<String>,
    /// Location country
    country:  String,
    /// Time zone name
    tz_id:  String,
    /// Local date and time in unix time
    localtime_epoch: u32,
    /// Local date and time
    localtime:  String
}

#[derive(RustcDecodable, Debug, Default)]
pub struct CurrentWeather {
    location: Location,
    current: Current
}

#[derive(RustcDecodable, Debug, Default)]
pub struct Current {
    /// Local time when the real time data was updated.
    last_updated: String,
    /// Local time when the real time data was updated in unix time.
    last_updated_epoch: i32,
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
    wind_degree: i32,
    /// Wind direction as 16 point compass. e.g.: NSW
    wind_dir: String,
    /// Pressure in millibars
    pressure_mb: f32,
    /// Pressure in inches
    pressure_in: f32,
    /// Precipitation amount in millimeters
    precip_mm: f32,
    /// Precipitation amount in inches
    precip_in: f32,
    /// Humidity as percentage
    humidity: u8,
    /// Cloud cover as percentage
    cloud: u8,
    /// Feels like temperature as celcius
    feelslike_c: f32,
    /// Feels like temperature as fahrenheit
    feelslike_f: f32,
    is_day: u8, // 1 = Yes 0 = No
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
            api_key: env::var("APIXU_API_KEY").expect("You need to the env var APIXU_API_KEY to be set.")
        };
        acfg
    };
}

// Build an Url to be used by Hyper.
fn mk_url(uri_path: &str, params: Vec<(&str, &str)>) -> String {
    let param_string: String = params.iter().fold(String::new(), |acc, &x| format!("{}&{}={}", acc, x.0, x.1));
    String::from(format!("{}{}?key={}{}", APIXU_URL, uri_path, APIXU_CFG.api_key, param_string))
}

/// Gets the current weather based on Auto IP.
// TODO: Better error handling.
pub fn current_weather(client: &hyper::client::Client, city: Option<&City>) -> Result<CurrentWeather, ApixuError> {
    let url = mk_url("current.json", vec![("q", city.unwrap_or("auto:ip"))]);
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
    #[ignore]
    fn can_decode_a_current_weather_request() {
        let client = Client::new();
        match current_weather(&client, Some("Marsala")) {
            Ok(cw) => {
                assert_eq!(cw.location.name, "Marsala");
                assert_eq!(cw.location.country, "Italy")
            },
            Err(e) => panic!(format!("{:?}", e)),
        }
    }
}
