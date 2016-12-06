
extern crate hyper;

use hyper::client::Client;
use std::io::Read;
use std::iter::FromIterator;
extern crate select;
use select::document::Document;
use select::node::Node;
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
        let tds:Vec<_> = node.find(pred).collect();

        /*
        let myvec = vec![1,2,3,4];
        let vec2:Vec<_> = myvec.iter().map(|x| x * 2).collect();

        println!("{:?}", myvec);
        println!("{:?}", vec2);
         */

        let mut images = Vec::new();
        for td in &tds {
            images.extend(td.find(Name("img")));
        }

        //let images:Vec<_> = tds0.flat_map(|t| t.find(Name("img")).collect::<Vec<_>>()).collect();
        //let text   = tds.nth(1).map(|t| t.find(Name("p")));
        for td in images {
                println!("--> {:?}", td.attr("src").unwrap_or(""))
        }
    }
}
