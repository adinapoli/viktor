
#[macro_use] extern crate lazy_static;

extern crate hyper;
extern crate select;
extern crate rustc_serialize;

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
    CliError(cli::CliParseError)
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
    let _ = client.get(runners_world::RUNNERS_WORLD_URL)
        .send()
        .map(|mut r| r.read_to_string(&mut body))
        .expect("Couldn't contact Runner's World website.");
    let document = Document::from(body.as_str());

    let weather = apixu_weather::current_weather(&client, args.city);

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
