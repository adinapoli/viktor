
#[macro_use]
extern crate lazy_static;

extern crate hyper;
extern crate select;
extern crate rustc_serialize;

use hyper::client::Client;

use std::env;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::process;

use rustc_serialize::base64::{ToBase64};
use rustc_serialize::base64;

use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Name};

mod apixu_weather;
mod runners_world;
mod cli;

static RUNNERS_WORLD_URL: &'static str = "http://www.runnersworld.com/what-to-wear?gender=m&temp=35&conditions=pc&wind=nw&time=dawn&intensity=n&feel=ib";

fn download_img(client: &hyper::client::Client, url: &str) -> Result<Vec<u8>, hyper::error::Error> {
    let mut body = Vec::new();
    client.get(url)
        .send()
        .map(|mut r| r.read_to_end(&mut body))
        .map(|_| body)
}

fn to_base_64(img: &[u8]) -> String {
    let cfg = base64::Config{ char_set: base64::Standard
                            , newline: base64::Newline::LF
                            , pad: true
                            , line_length: None };
    img.to_base64(cfg)
}

#[derive(Debug)]
enum TerminalInUse {
    TmuxOrScreen,
    StandardTerminal
}

fn user_terminal() -> TerminalInUse {
    match env::var("TERM") {
        Err(_) => return TerminalInUse::StandardTerminal,
        Ok(ref s) => if s.contains("screen") { return TerminalInUse::TmuxOrScreen }
    }
    return TerminalInUse::StandardTerminal
}

// TODO: This doesn't display nicely at all inside tmux, we neeed to find
// a workaround. One could be to use https://github.com/PistonDevelopers/image
// to load the image in memory and get its dimension, and using explicit height
// in the iTerm inline capability.
fn display_inline_image(img_mb: Result<String, hyper::error::Error>) {
    match img_mb {
        Err(_) => return,
        Ok(i) => {
            let (initial_seq, final_seq) = match user_terminal() {
                TerminalInUse::TmuxOrScreen => ("\x1BPtmux;\x1B\x1B]", "\x07\x1B\\"),
                TerminalInUse::StandardTerminal => ("\x1B]", "\x07")
            };
            println!("{}1337;File=inline=1:{}{}", initial_seq, i, final_seq);
        }
    }
}

fn find_images<'a>(tds: &'a Vec<Node>) -> HashSet<&'a str> {
    let mut images0 = Vec::new();
    for td in tds {
        images0.extend(td.find(Name("img")).filter_map(|x| x.attr("src").and_then(filter_image)));
    }

    HashSet::from_iter(images0)
}

fn filter_image(img: &str) -> Option<&str> {
    if img.contains("logo") { None } else { Some(img) }
}

fn find_descriptions(tds: &Vec<Node>) -> HashSet<String> {
    let mut descs = Vec::new();
    for td in tds {
        descs.extend(td.find(Name("p")).filter_map(|x| filter_description(x.text())));
    }

    HashSet::from_iter(descs)
}

fn filter_description(d: String) -> Option<String> {
    if d.is_empty() || d.contains("Revise Conditions") {
        None
    } else {
        Some(d)
    }
}

#[derive(Debug)]
enum AppError {
    CliError(cli::CliParseError),
    GenericError
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
    let _ = client.get(RUNNERS_WORLD_URL)
        .send()
        .map(|mut r| r.read_to_string(&mut body))
        .expect("Couldn't contact Runner's World website.");
    let document = Document::from(body.as_str());

    let weather = apixu_weather::current_weather(&client, args.city);

    for node in document.find(Attr("id", "content")) {
        let pred = Name("table").descendant(Name("table").descendant(Name("td")));
        let tds:Vec<_> = node.find(pred).collect();

        let images = find_images(&tds);
        for img in images {
            println!("--> {:?}", img);
            display_inline_image(download_img(&client, img).map(|x| to_base_64(&x)));
        }

        let descriptions = find_descriptions(&tds);
        for desc in descriptions {
            println!("--> {:?}", desc);
        }
    }

    Ok(())
}

#[test]
#[ignore]
fn can_download_img() {
    let client = Client::new();
    let mut body = Vec::new();
    let _ = client.get("http://www.runnersworld.com/sites/runnersworld.com/modules/custom/rw_what_to_wear/images/logo.png")
        .send()
        .map(|mut r| r.read_to_end(&mut body))
        .expect("Couldn't contact Runner's World website.");
    assert!(body.len() != 0);
}
