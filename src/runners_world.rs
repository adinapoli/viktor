
extern crate hyper;
extern crate rustc_serialize;

use hyper::client::Client;
use std::env;
use std::num;
use std::fmt;
use std::io::Read;
use rustc_serialize::base64::{ToBase64};
use std::iter::FromIterator;
use rustc_serialize::base64;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Name};
use std::collections::HashSet;


pub struct FormBuilder {
    gender: String,
    temperature: String,
    conditions: String,
    wind: String,
    time_of_day: String,
    intensity: String,
    feel: String,
}

pub fn form_builder() -> FormBuilder {
    FormBuilder {
        gender: "male".to_owned(),
        temperature: "10".to_owned(),
        conditions: "10".to_owned(),
        wind: "10".to_owned(),
        time_of_day: "10".to_owned(),
        intensity: "10".to_owned(),
        feel: "10".to_owned(),
    }
}

#[derive(Eq, Debug, PartialEq, Hash)]
pub struct Image<'a> {
    pub url: &'a str,
    width: TermDimension,
    height: TermDimension,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum TermDimension {
    Auto,
    Dimension(u8)
}

impl fmt::Display for TermDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TermDimension::Auto => write!(f, "auto"),
            TermDimension::Dimension(d) => write!(f, "{}", d)
        }
    }
}

pub static RUNNERS_WORLD_URL: &'static str = "http://www.runnersworld.com/what-to-wear?gender=m&temp=35&conditions=pc&wind=nw&time=dawn&intensity=n&feel=ib";

pub fn download_img(client: &hyper::client::Client, url: &str) -> Result<Vec<u8>, hyper::error::Error> {
    let mut body = Vec::new();
    client.get(url)
        .send()
        .map(|mut r| r.read_to_end(&mut body))
        .map(|_| body)
}

pub fn to_base_64(img: &[u8]) -> String {
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
pub fn display_inline_image(img_info: &Image, img_mb: Result<String, hyper::error::Error>) {
    match img_mb {
        Err(_) => return,
        Ok(i) => {
            let (initial_seq, final_seq) = match user_terminal() {
                TerminalInUse::TmuxOrScreen => ("\x1BPtmux;\x1B\x1B]", "\x07\x1B\\"),
                TerminalInUse::StandardTerminal => ("\x1B]", "\x07")
            };
            println!("{}1337;File=inline=1;width={}px;height={}px:{}{}", initial_seq, img_info.width, img_info.height, i, final_seq);
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
        Some(i) => if i.contains("logo") {
            None
        } else {
            Some(Image {
                url: i,
                width: img_node.attr("width")
                    .ok_or("not found".to_owned())
                    .and_then(|v| str::parse(v).map_err(|e:num::ParseIntError| e.to_string()))
                    .map(TermDimension::Dimension)
                    .unwrap_or(TermDimension::Auto),
                height: img_node.attr("height")
                    .ok_or("not found".to_owned())
                    .and_then(|v| str::parse(v).map_err(|e:num::ParseIntError| e.to_string()))
                    .map(TermDimension::Dimension)
                    .unwrap_or(TermDimension::Auto),
            })
        }
    }
}

pub fn find_descriptions(tds: &Vec<Node>) -> HashSet<String> {
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
