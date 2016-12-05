
extern crate hyper;

use hyper::client::Client;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Predicate, Attr, Name};

static RUNNERS_WORLD_URL: &'static str = "http://www.runnersworld.com/what-to-wear?gender=m&temp=35&conditions=pc&wind=nw&time=dawn&intensity=n&feel=ib";

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
        let tds = node.find(pred).collect::<Vec<_>>();
        let images = tds.map(|t| t.find(Name("p")));
        for td in tds {
            println!("--> {:?}", td.html())
        }
    }
}
