#![feature(proc_macro)]
#![feature(slice_patterns)]

extern crate hyper;
extern crate select;
extern crate termcolor;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use hyper::client::Client;

use std::io::Read;
use std::process;
use std::collections::HashSet;

use select::document::Document;
use select::predicate::{Predicate, Attr, Name};
use std::iter::FromIterator;
use std::iter::IntoIterator;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, Stdout, WriteColor};

mod apixu_weather;
mod runners_world;
mod cli;

#[derive(Debug)]
enum AppError {
    CliError(cli::CliParseError),
    GenericError(std::string::String)
}

impl From<apixu_weather::ApixuError> for AppError {
    fn from(err: apixu_weather::ApixuError) -> AppError {
        AppError::GenericError(format!("{:?}", err))
    }
}

fn show_visual_recap(args: &cli::Args, weather: &apixu_weather::CurrentWeather) -> Result<(), Box<::std::error::Error>>{
    let mut stdout = Stdout::new(ColorChoice::Always);
    println!("\n");

    // Gender
    try!(write!(&mut stdout, "Gender: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))));
    try!(writeln!(&mut stdout, "{}", args.gender));
    try!(stdout.reset());

    // Intensity
    try!(write!(&mut stdout, "Intensity: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))));
    try!(writeln!(&mut stdout, "{}", args.intensity));
    try!(stdout.reset());

    // City
    try!(write!(&mut stdout, "City: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))));
    let city = args.city.clone();
    let city_name = weather.location.name.clone();
    let country   = weather.location.country.clone();
    try!(writeln!(&mut stdout, "{}", city.unwrap_or(city_name + ", " + &country.to_string() + " (Inferred)")));
    try!(stdout.reset());

    // Temp
    try!(write!(&mut stdout, "Temperature now: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))));
    let temp_c = weather.current.temp_c;
    let temp_f = weather.current.temp_f;
    try!(writeln!(&mut stdout, "{}C ({}F)", temp_c, temp_f));
    try!(stdout.reset());

    // Weather
    try!(write!(&mut stdout, "Weather now: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))));
    let condition = &weather.current.condition.text;
    try!(writeln!(&mut stdout, "{}", condition));
    try!(stdout.reset());

    // Wind
    try!(write!(&mut stdout, "Wind: "));
    try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))));
    let wind_kph = &weather.current.wind_kph;
    let wind_mph = &weather.current.wind_mph;
    try!(writeln!(&mut stdout, "{} kph ({} mph)", wind_kph, wind_mph));
    try!(stdout.reset());


    // Finalise everything
    println!("\n");
    Ok(())
}

fn print_descriptions(descriptions: HashSet<(String, String)>) -> Result<(), Box<::std::error::Error>> {

    let mut stdout = Stdout::new(ColorChoice::Always);
    println!("");
    for (item, desc) in descriptions {
        try!(stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))));
        try!(writeln!(&mut stdout, "{}", item));
        try!(stdout.reset());
        try!(stdout.set_color(ColorSpec::new().set_bold(true)));
        try!(writeln!(&mut stdout, "{}", desc));
        try!(stdout.reset());
    }

    Ok(())
}

fn main() {
   match cli::Args::parse().map_err(AppError::CliError).and_then(run) {
        Ok(()) => process::exit(0),
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
   }
}

fn run(args: cli::Args) -> Result<(), AppError> {
    let client = Client::new();
    let mut body = String::new();

    let weather = try!(apixu_weather::current_weather(&client, &args.city));
    let form_builder = runners_world::FormBuilder::new(&args.gender, &args.intensity, &weather);
    let url = form_builder.to_url().to_string();

    let _ = client.get(&url)
        .send()
        .map(|mut r| r.read_to_string(&mut body))
        .expect("Couldn't contact Runner's World website.");
    let document = Document::from(body.as_str());

    // Show a visual recap
    let _ = show_visual_recap(&args, &weather);

    for node in document.find(Attr("id", "content")) {
        let pred = Name("table").descendant(Name("table").descendant(Name("td")));
        let tds:Vec<_> = node.find(pred).collect();

        let images = runners_world::find_images(&tds);
        let mut images_sorted = Vec::from_iter(images);
        images_sorted.sort();
        for img_info in images_sorted {
            runners_world::display_inline_image(&img_info,
                                                runners_world::download_img(&client, img_info.url)
                                                .map(|x| runners_world::to_base_64(&x))
            );
        }

        let descriptions = runners_world::find_descriptions(&tds);
        let _ = print_descriptions(descriptions);
    }

    Ok(())
}
