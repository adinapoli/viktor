#![feature(proc_macro)]
#![feature(slice_patterns)]

extern crate hyper;
extern crate select;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use hyper::client::Client;

use std::io::Read;
use std::process;

use select::document::Document;
use select::predicate::{Predicate, Attr, Name};
use std::iter::FromIterator;

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

    let weather = try!(apixu_weather::current_weather(&client, args.city));
    let form_builder = runners_world::FormBuilder::new(&args.gender, &args.intensity, &weather);
    let url = form_builder.to_url().to_string();

    let _ = client.get(&url)
        .send()
        .map(|mut r| r.read_to_string(&mut body))
        .expect("Couldn't contact Runner's World website.");
    let document = Document::from(body.as_str());

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
        for desc in descriptions {
            println!("--> {:?}", desc);
        }
    }

    Ok(())
}
