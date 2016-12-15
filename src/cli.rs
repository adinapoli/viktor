
extern crate clap;

use cli::clap::{Arg, App};
use std;

#[derive(Debug)]
pub enum CliParseError {
    ParseGenderError(std::string::String),
    ParseIntensityError(std::string::String),
}

#[derive(Debug)]
pub struct Args {
    pub gender: Gender,
    pub intensity: Intensity,
    pub city: Option<String>,
}

// TODO: This is horrid, we should be able to use lifetime specifier
// to avoid the conversion to String.
impl Args {
    pub fn parse() -> Result<Args, CliParseError> {
        let matches = cli().get_matches();
        let gender = try!(matches.value_of("gender")
            .ok_or(CliParseError::ParseGenderError(String::from("Gender is required.")))
            .and_then(parse_gender));
        let intensity = try!(matches.value_of("intensity")
            .ok_or(CliParseError::ParseIntensityError(String::from("Intensity is required.")))
            .and_then(parse_intensity));
        let args = Args {
            gender: gender,
            intensity: intensity,
            city: matches.value_of("city").map(String::from),
        };
        Ok(args)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Intensity {
    EasyRun,
    LongRun,
    HardWorkout,
    Race,
}

fn parse_gender(input: &str) -> Result<Gender, CliParseError> {
    match input {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(CliParseError::ParseGenderError(String::from(input))),
    }
}

fn parse_intensity(input: &str) -> Result<Intensity, CliParseError> {
    match input {
        "easy_run" => Ok(Intensity::EasyRun),
        "long_run" => Ok(Intensity::LongRun),
        "hard_workout" => Ok(Intensity::HardWorkout),
        "race" => Ok(Intensity::Race),
        _ => Err(CliParseError::ParseIntensityError(String::from(input))),
    }
}

pub fn cli() -> App<'static, 'static> {
    let gender_arg = Arg::with_name("gender")
        .long("gender")
        .short("g")
        .value_name("male|female")
        .help("Your gender ('male' or 'female')")
        .required(true);
    let city_arg = Arg::with_name("city")
        .long("city")
        .short("c")
        .value_name("CITY")
        .help("The city you are in right now.")
        .required(false);
    let intensity_arg = Arg::with_name("intensity")
        .long("intensity")
        .short("i")
        .value_name("INTENSITY")
        .help("easy run|long run|hard workout|race")
        .required(true);
    let app = App::new("Viktor")
        .version("0.0.1")
        .author("Alfredo Di Napoli")
        .about("Display on iTerm what to wear while running")
        .arg(city_arg)
        .arg(intensity_arg)
        .arg(gender_arg);
    app
}
