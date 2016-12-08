
extern crate hyper;
extern crate select;
extern crate rustc_serialize;

use hyper::client::Client;

use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashSet;

use rustc_serialize::base64::{ToBase64};
use rustc_serialize::base64;

use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Name};

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

fn main() {
    let client = Client::new();
    let mut body = String::new();
    let _ = client.get(RUNNERS_WORLD_URL)
        .send()
        .map(|mut r| r.read_to_string(&mut body))
        .expect("Couldn't contact Runner's World website.");
    let document = Document::from(body.as_str());

    for node in document.find(Attr("id", "content")) {
        let pred = Name("table").descendant(Name("table").descendant(Name("td")));
        let tds:Vec<_> = node.find(pred).collect();

        let mut images0 = Vec::new();
        for td in &tds {
            images0.extend(td.find(Name("img")).map(|x| x.attr("src").unwrap_or("")));
        }

        let images:HashSet<_> = HashSet::from_iter(images0);

        //let images:Vec<_> = tds0.flat_map(|t| t.find(Name("img")).collect::<Vec<_>>()).collect();
        //let text   = tds.nth(1).map(|t| t.find(Name("p")));
        for td in images {
            // TODO: Proper Error handling, avoid unwrap_or
            let img_url = td;
            println!("--> {:?}", img_url);
            let res = download_img(&client, img_url).map(|x| to_base_64(&x));
            print!("\x1B]1337;File=inline=1:{}\x07", res.unwrap_or(String::from("")));
            print!("\n");
        }
    }
}

#[test]
fn can_download_img() {
    let client = Client::new();
    let mut body = Vec::new();
    let _ = client.get("http://www.runnersworld.com/sites/runnersworld.com/modules/custom/rw_what_to_wear/images/logo.png")
        .send()
        .map(|mut r| r.read_to_end(&mut body))
        .expect("Couldn't contact Runner's World website.");
    assert!(body.len() != 0);
}
