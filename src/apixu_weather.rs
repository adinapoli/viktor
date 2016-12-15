
extern crate serde_json;
extern crate termcolor;
extern crate hyper;

pub use hyper::client::Client;
use std::io::Read;
use std;
use std::process;
use std::env;
use std::io::Write;
use self::termcolor::{Color, ColorChoice, ColorSpec, Stdout, WriteColor};

static APIXU_URL: &'static str = "https://api.apixu.com/v1/";
static WEATHER_CONDITIONS: [(&'static str, u32); 48] =
    [("Sunny", 1000),
     ("Partly Cloudy", 1003),
     ("Cloudy", 1006),
     ("Overcast", 1009),
     ("Mist", 1030),
     ("Patchy rain nearby", 1063),
     ("Patchy snow nearby", 1066),
     ("Patchy sleet nearby", 1069),
     ("Patchy freezing drizzle nearby", 1072),
     ("Thundery outbreaks in nearby", 1087),
     ("Blowing snow", 1114),
     ("Blizzard", 1117),
     ("Fog", 1135),
     ("Freezing fog", 1147),
     ("Patchy light drizzle", 1150),
     ("Light drizzle", 1153),
     ("Freezing drizzle", 1168),
     ("Heavy freezing drizzle", 1171),
     ("Patchy light rain", 1180),
     ("Light rain", 1183),
     ("Moderate rain at times", 1186),
     ("Moderate rain", 1189),
     ("Heavy rain at times", 1192),
     ("Heavy rain", 1195),
     ("Light freezing rain", 1198),
     ("Moderate or heavy freezing rain", 1201),
     ("Light sleet", 1204),
     ("Moderate or heavy sleet", 1207),
     ("Patchy light snow", 1210),
     ("Light snow", 1213),
     ("Patchy moderate snow", 1216),
     ("Moderate snow", 1219),
     ("Patchy heavy snow", 1222),
     ("Heavy snow", 1225),
     ("Ice pellets", 1237),
     ("Light rain shower", 1240),
     ("Moderate or heavy rain shower", 1243),
     ("Torrential rain shower", 1246),
     ("Light sleet showers", 1249),
     ("Moderate or heavy sleet showers", 1252),
     ("Light snow showers", 1255),
     ("Moderate or heavy snow showers", 1258),
     ("Light showers of ice pellets", 1261),
     ("Moderate or heavy showers of ice pellets", 1264),
     ("Patchy light rain in area with thunder", 1273),
     ("Moderate or heavy rain in area with thunder", 1276),
     ("Patchy light snow in area with thunder", 1279),
     ("Moderate or heavy snow in area with thunder", 1282)];

struct ApixuCfg {
    api_key: String,
}

type City = String;

#[derive(Deserialize, Debug, Default)]
pub struct Location {
    /// Latitude in decimal degree
    lat: f32,
    /// Longitude in decimal degree
    lon: f32,
    /// Location name
    pub name: String,
    /// Region or state of the location, if available
    region: Option<String>,
    /// Location country
    pub country: String,
    /// Time zone name
    tz_id: String,
    /// Local date and time in unix time
    localtime_epoch: u32,
    /// Local date and time
    localtime: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct CurrentWeather {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize, Debug, Default)]
pub struct WeatherCondition {
    pub text: String,
    pub code: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct Current {
    /// Local time when the real time data was updated.
    pub last_updated: String,
    /// Local time when the real time data was updated in unix time.
    last_updated_epoch: i32,
    /// Temperature in celsius
    pub temp_c: f32,
    /// Temperature in fahrenheit
    pub temp_f: f32,
    // condition:text	string	Weather condition text
    // condition:icon	string	Weather icon url
    pub condition: WeatherCondition,
    /// Wind speed in miles per hour
    pub wind_mph: f32,
    /// Wind speed in kilometer per hour
    pub wind_kph: f32,
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
    pub feelslike_c: f32,
    /// Feels like temperature as fahrenheit
    feelslike_f: f32,
    is_day: u8, // 1 = Yes 0 = No
}

#[derive(Debug)]
pub enum ApixuError {
    FailedToContactRemoteHost(hyper::error::Error),
    InvalidRequest(String, hyper::client::Response),
    IOError(std::io::Error),
    ParseJsonError(serde_json::error::Error),
}

impl From<hyper::error::Error> for ApixuError {
    fn from(err: hyper::error::Error) -> ApixuError {
        ApixuError::FailedToContactRemoteHost(err)
    }
}

impl From<serde_json::error::Error> for ApixuError {
    fn from(err: serde_json::error::Error) -> ApixuError {
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
        ApixuCfg { api_key: get_apixu_key() }
    };
}

// TODO: Investigate how/if termcolor supports stderr.
fn get_apixu_key() -> String {
    match env::var("APIXU_API_KEY") {
        Err(_) => {
            let mut stdout = Stdout::new(ColorChoice::Always);
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
            let _ = writeln!(&mut stdout,
                             r###"
You need to the env var APIXU_API_KEY to be set. (Try 'export APIXU_API_KEY=xxxxxx...')
Please go to "https://www.apixu.com/" and create a new free account in
order to get a valid API key for the weather service.

Visit the "Prerequisites" section of the README for more information.
"###);
            process::exit(1);
        }
        Ok(v) => v,
    }
}

// Build an Url to be used by Hyper.
fn mk_url(uri_path: &str, params: Vec<(&str, &String)>) -> String {
    let param_string: String = params.iter()
        .fold(String::new(), |acc, &x| format!("{}&{}={}", acc, x.0, x.1));
    String::from(format!("{}{}?key={}{}",
                         APIXU_URL,
                         uri_path,
                         APIXU_CFG.api_key,
                         param_string))
}

/// Gets the current weather based on Auto IP.
// TODO: Better error handling.
pub fn current_weather(client: &hyper::client::Client,
                       city: &Option<City>)
                       -> Result<CurrentWeather, ApixuError> {
    let the_city = city.clone().unwrap_or("auto:ip".to_owned());
    let url = mk_url("current.json", vec![("q", &the_city)]);
    let mut response = try!(client.get(&url).send());
    if response.status != hyper::status::StatusCode::Ok {
        return Err(ApixuError::InvalidRequest(url, response));
    }
    let mut body = String::new();
    let _ = try!(response.read_to_string(&mut body));
    let cw: CurrentWeather = try!(serde_json::from_str(&body));
    Ok(cw)
}

pub fn parse_hours_from_last_updated<'a>(last_updated: &'a str) -> Option<u8> {
    match *last_updated.to_owned().split_whitespace().collect::<Vec<_>>().as_slice() {
        [_, time] => {
            match *time.split(":").collect::<Vec<_>>().as_slice() {
                [h, _] => return str::parse(h).ok(),
                _ => return None,
            }
        }
        _ => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn can_decode_a_current_weather_request() {
        let client = Client::new();
        match current_weather(&client, &Some("Marsala".to_owned())) {
            Ok(cw) => {
                assert_eq!(cw.location.name, "Marsala");
                assert_eq!(cw.location.country, "Italy")
            }
            Err(e) => panic!(format!("{:?}", e)),
        }
    }

    #[test]
    fn can_parse_last_updated_field_into_hours() {
        let test1 = "2016-12-15 09:22";
        let test2 = "2016-12-15 3:24";
        assert_eq!(parse_hours_from_last_updated(&test1), Some(9));
        assert_eq!(parse_hours_from_last_updated(&test2), Some(3));
    }
}
