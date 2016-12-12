
extern crate clap;

use cli::clap::{Arg, App};
use std;

#[derive(Debug)]
pub enum CliParseError {
    ParseGenderError(std::string::String),
}

#[derive(Debug)]
pub struct Args {
    gender: Gender,
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
        let args = Args {
            gender: gender,
            city: matches.value_of("city").map(String::from),
        };
        Ok(args)
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

fn parse_gender(input: &str) -> Result<Gender, CliParseError> {
    match input {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(CliParseError::ParseGenderError(String::from(input))),
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
    let app = App::new("Runiterm")
        .version("0.0.1")
        .author("Alfredo Di Napoli")
        .about("Display on iTerm what to wear while running")
        .arg(city_arg)
        .arg(gender_arg);
    app
}
