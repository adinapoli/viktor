
extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::num;
use std::fmt;
use std::io::Read;
use std::cmp::{Ord, Ordering};
use self::rustc_serialize::base64::ToBase64;
use std::iter::FromIterator;
use self::rustc_serialize::base64;
use select::node::Node;
use select::predicate::Name;
use std::collections::HashSet;

use apixu_weather::{CurrentWeather, parse_hours_from_last_updated};
use cli::{Gender, Intensity};

pub static RUNNERS_WORLD_URL: &'static str = "http://www.runnersworld.com/what-to-wear";

pub struct FormBuilder {
    gender: &'static str,
    temperature: &'static str,
    conditions: &'static str,
    wind: &'static str,
    time_of_day: &'static str,
    intensity: &'static str,
    feel: &'static str,
}

impl FormBuilder {
    pub fn to_url(&self) -> String {
        return format!("{}?gender={}&temp={}&conditions={}&wind={}&time={}&intensity={}&feel={}",
                       RUNNERS_WORLD_URL,
                       self.gender,
                       self.temperature,
                       self.conditions,
                       self.wind,
                       self.time_of_day,
                       self.intensity,
                       self.feel);
    }

    pub fn new(gender: &Gender, intensity: &Intensity, weather: &CurrentWeather) -> FormBuilder {
        FormBuilder {
            gender: if *gender == Gender::Male { "m" } else { "f" },
            temperature: FormBuilder::to_temperature(weather),
            conditions: FormBuilder::to_conditions(weather),
            wind: FormBuilder::to_wind(weather),
            time_of_day: FormBuilder::to_time_of_day(weather),
            intensity: FormBuilder::to_intensity(intensity),
            feel: FormBuilder::to_feel(weather),
        }
    }

    fn to_time_of_day(w: &CurrentWeather) -> &'static str {
        match parse_hours_from_last_updated(&w.current.last_updated.to_string()) {
            None => return "day", //Assume day by default
            Some(h) => {
                if w.current.is_day == 0 {
                    return "night";
                }
                if h < 8 {
                    return "dawn";
                }
                if h > 8 && h < 16 {
                    return "day";
                }
                if h > 16 && h < 20 {
                    return "dusk";
                }
                return "night";
            }
        }
    }

    // Loosely based on: https://www.windfinder.com/wind/windspeed.htm
    fn to_wind(w: &CurrentWeather) -> &'static str {
        if w.current.wind_mph <= 3.0 {
            return "nw";
        }
        if w.current.wind_mph > 3.0 && w.current.wind_mph < 25.0 {
            return "lw";
        }
        if w.current.wind_mph > 25.0 {
            return "hw";
        }
        // Assume no wind by default
        return "nw";
    }

    // TODO: Make more sophisticate
    fn to_feel(w: &CurrentWeather) -> &'static str {
        let real_temp = w.current.temp_c;
        let feel_temp = w.current.feelslike_c;

        if real_temp == feel_temp {
            return "ib";
        }
        if real_temp < (feel_temp + 10.0) {
            return "w";
        }
        if real_temp > (feel_temp + 10.0) {
            return "c";
        }

        // Assume in-between otherwise
        return "ib";
    }

    fn to_conditions(w: &CurrentWeather) -> &'static str {
        let weather_code = w.current.condition.code;
        if weather_code == 1000 {
            return "c";
        };
        if weather_code >= 1003 {
            return "pc";
        };
        if weather_code > 1003 && weather_code <= 1009 {
            return "o";
        };
        if weather_code == 1183 {
            return "lr";
        };
        if weather_code != 1183 && weather_code > 1009 && weather_code < 1195 {
            return "r";
        };
        // Assume snow otherwise
        return "s";
    }

    fn to_intensity(i: &Intensity) -> &'static str {
        match *i {
            Intensity::EasyRun => return "n",
            Intensity::LongRun => return "lr",
            Intensity::HardWorkout => return "h",
            Intensity::Race => return "r",
        }
    }

    fn to_temperature(w: &CurrentWeather) -> &'static str {
        let temp = w.current.temp_f;
        if temp < -5.0 {
            return "-10";
        }
        if temp >= -5.0 && temp < 0.0 {
            return "-5";
        }
        if temp >= 0.0 && temp < 5.0 {
            return "zero";
        }
        if temp >= 5.0 && temp < 10.0 {
            return "5";
        }
        if temp >= 10.0 && temp < 15.0 {
            return "10";
        }
        if temp >= 15.0 && temp < 20.0 {
            return "15";
        }
        if temp >= 20.0 && temp < 25.0 {
            return "20";
        }
        if temp >= 25.0 && temp < 30.0 {
            return "25";
        }
        if temp >= 30.0 && temp < 35.0 {
            return "30";
        }
        if temp >= 35.0 && temp < 40.0 {
            return "35";
        }
        if temp >= 40.0 && temp < 45.0 {
            return "40";
        }
        if temp >= 45.0 && temp < 50.0 {
            return "45";
        }
        if temp >= 50.0 && temp < 55.0 {
            return "50";
        }
        if temp >= 55.0 && temp < 60.0 {
            return "55";
        }
        if temp >= 60.0 && temp < 65.0 {
            return "60";
        }
        if temp >= 65.0 && temp < 70.0 {
            return "65";
        }
        if temp >= 70.0 && temp < 75.0 {
            return "70";
        }
        if temp >= 75.0 && temp < 80.0 {
            return "75";
        }
        if temp >= 80.0 && temp < 85.0 {
            return "80";
        }
        if temp >= 85.0 && temp < 90.0 {
            return "85";
        }
        if temp >= 90.0 && temp < 95.0 {
            return "90";
        }
        if temp >= 95.0 && temp < 100.0 {
            return "95";
        }
        if w.current.temp_f >= 100.0 {
            return "100";
        }
        return "100";
    }
}

#[derive(Eq, Debug, PartialOrd, PartialEq, Hash)]
pub struct Image<'a> {
    pub url: &'a str,
    width: TermDimension,
    height: TermDimension,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Hash)]
enum TermDimension {
    Auto,
    Dimension(u8),
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
enum BodyPart {
    Head,
    Torso,
    Legs,
    Feet,
}

impl<'a> Image<'a> {
    fn body_part(&self) -> Option<BodyPart> {
        if self.url.contains("head") {
            return Some(BodyPart::Head);
        }
        if self.url.contains("torso") {
            return Some(BodyPart::Torso);
        }
        if self.url.contains("legs") {
            return Some(BodyPart::Legs);
        }
        if self.url.contains("feet") {
            return Some(BodyPart::Feet);
        }
        return None;
    }
}

// Order an image according to the part of the body: head, legs, torso, feet.
impl<'a> Ord for Image<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.body_part(), other.body_part()) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(a), Some(b)) => a.cmp(&b),
        }
    }
}

impl fmt::Display for TermDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TermDimension::Auto => write!(f, "auto"),
            TermDimension::Dimension(d) => write!(f, "{}", d),
        }
    }
}

pub fn download_img(client: &hyper::client::Client,
                    url: &str)
                    -> Result<Vec<u8>, hyper::error::Error> {
    let mut body = Vec::new();
    client.get(url)
        .send()
        .map(|mut r| r.read_to_end(&mut body))
        .map(|_| body)
}

pub fn to_base_64(img: &[u8]) -> String {
    let cfg = base64::Config {
        char_set: base64::Standard,
        newline: base64::Newline::LF,
        pad: true,
        line_length: None,
    };
    img.to_base64(cfg)
}

#[derive(Debug)]
enum TerminalInUse {
    TmuxOrScreen,
    StandardTerminal,
}

fn user_terminal() -> TerminalInUse {
    match env::var("TERM") {
        Err(_) => return TerminalInUse::StandardTerminal,
        Ok(ref s) => {
            if s.contains("screen") {
                return TerminalInUse::TmuxOrScreen;
            }
        }
    }
    return TerminalInUse::StandardTerminal;
}

// TODO: This doesn't display nicely at all inside tmux, we neeed to find
// a workaround. One could be to use https://github.com/PistonDevelopers/image
// to load the image in memory and get its dimension, and using explicit height
// in the iTerm inline capability.
pub fn display_inline_image(img_info: &Image, img_mb: Result<String, hyper::error::Error>) {
    match img_mb {
        Err(_) => return,
        Ok(i) => {
            let (initial_seq, final_seq) = match user_terminal() {
                TerminalInUse::TmuxOrScreen => ("\x1BPtmux;\x1B\x1B]", "\x07\x1B\\"),
                TerminalInUse::StandardTerminal => ("\x1B]", "\x07"),
            };
            println!("{}1337;File=inline=1;width={}px;height={}px:{}{}",
                     initial_seq,
                     img_info.width,
                     img_info.height,
                     i,
                     final_seq);
        }
    }
}

pub fn find_images<'a>(tds: &'a Vec<Node>) -> HashSet<Image<'a>> {
    let mut images0 = Vec::new();
    for td in tds {
        images0.extend(td.find(Name("img")).filter_map(|x| mk_image(&x)));
    }

    HashSet::from_iter(images0)
}

fn mk_image<'a>(img_node: &Node<'a>) -> Option<Image<'a>> {
    match img_node.attr("src") {
        None => None,
        Some(i) => {
            if i.contains("logo") {
                None
            } else {
                Some(Image {
                    url: i,
                    width: img_node.attr("width")
                        .ok_or("not found".to_owned())
                        .and_then(|v| str::parse(v).map_err(|e: num::ParseIntError| e.to_string()))
                        .map(TermDimension::Dimension)
                        .unwrap_or(TermDimension::Auto),
                    height: img_node.attr("height")
                        .ok_or("not found".to_owned())
                        .and_then(|v| str::parse(v).map_err(|e: num::ParseIntError| e.to_string()))
                        .map(TermDimension::Dimension)
                        .unwrap_or(TermDimension::Auto),
                })
            }
        }
    }
}

pub fn find_descriptions(tds: &Vec<Node>) -> HashSet<(String, String)> {
    let mut descs = Vec::new();
    for td in tds.iter().skip(1) {
        descs.extend(td.find(Name("p")).filter_map(|x| filter_description(x)));
    }

    HashSet::from_iter(descs)
}

fn filter_description(d: Node) -> Option<(String, String)> {
    match (d.first_child().map(|v| v.text()), d.last_child().map(|v| v.text())) {
        (Some(item), Some(desc)) => {
            if desc.is_empty() || desc.contains("Revise Conditions") {
                None
            } else {
                Some((item, desc))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::client::Client;
    #[test]
    #[ignore]
    fn can_download_img() {
        let client = Client::new();
        let mut body = Vec::new();
        let _ = client.get("http://www.runnersworld.com/sites/runnersworld.\
                  com/modules/custom/rw_what_to_wear/images/logo.png")
            .send()
            .map(|mut r| r.read_to_end(&mut body))
            .expect("Couldn't contact Runner's World website.");
        assert!(body.len() != 0);
    }
}
