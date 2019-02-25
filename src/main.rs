extern crate serde;
extern crate serde_json;
extern crate classifier;
extern crate regex;

use std::env;

mod domain;
mod tokenization;
mod training;
mod classification;

use crate::classification::classifier;

fn main() {
    let args: Vec<String> = env::args().collect();
    let classify = classifier(args.get(1).unwrap().to_owned());

    println!("{:#?}", classify("https://www.theguardian.com/sport/2019/feb/22/zion-williamson-injury-duke-nike-hypocrisy".to_owned()));
    println!("{:#?}", classify("https://www.nytimes.com/2019/02/21/world/asia/china-handwriting-robot.html".to_owned()));
    println!("{:#?}", classify("https://bgr.com/2019/02/20/moon-photo-50000-photos-andrew-mccarthy/".to_owned()));
}
