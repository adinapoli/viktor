
extern crate clap;

use cli::clap::{Arg, App, ArgMatches};
use std;

#[derive(Debug)]
pub enum CliParseError {
    ParseGenderError(std::string::String),
}

#[derive(Debug)]
pub struct Args {
    gender: String,
    city: Option<String>,
}

// TODO: This is horrid, we should be able to use lifetime specifier
// to avoid the conversion to String.
impl Args {
    pub fn parse() -> Result<Args, CliParseError> {
        let matches = cli().get_matches();
        Ok(Args {
            gender: String::from(matches.value_of("gender").unwrap_or("male")),
            city: matches.value_of("city").map(String::from),
        })
    }
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

pub fn cli() -> App<'static, 'static> {
    let gender_arg = Arg::with_name("gender")
        .long("gender")
        .short("g")
        .value_name("(male|female)")
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
