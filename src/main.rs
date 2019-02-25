#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate classifier;
extern crate regex;

use regex::Regex;
use serde_json::Result;
use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use classifier::NaiveBayes;

#[derive(Serialize, Deserialize, Debug)]
struct TraningUrl {
    url: String,
    category: String,
    title: Option<String>,
}

impl TraningUrl {
  fn get_tokens(&self) -> String {
    let title = self.title.clone().unwrap_or(" ".to_string());
    let mut input = self.url.clone();
    input.push_str(" ");
    input.push_str(&title);
    return tokenize(input);
  }
}

fn read_training_examples(training_path: String) -> Result<Vec<TraningUrl>> {
    let mut file = File::open(training_path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read error");

    let data: Vec<TraningUrl> = serde_json::from_str(&contents).expect("json parse error");

    Ok(data)
}

fn prepare_documents(examples: Vec<TraningUrl>) -> HashMap<String, String> {
    let mut documents: HashMap<String, String> = HashMap::new();

    for example in examples {
      let tokens = example.get_tokens();

      if !documents.contains_key(&example.category) {
        documents.insert(example.category.to_owned(), "".to_owned());
      }

      let document_tokens = documents.get_mut(&example.category).unwrap();

      document_tokens.push_str(" ");
      document_tokens.push_str(&tokens);
    }

    return documents;
}

fn tokenize(input: String) -> String {
    let re = Regex::new(r"[|\s,-_/?=&#:’`]+").expect("reg err");
    let mut fields: Vec<&str> = re.split(&input).collect();
    fields.retain(|&x| x != "http" && x != "https" && x != "www" && x != "com");
    return fields.join(" ");
}

fn train(training_path: String) -> Box<Fn(String) -> String> {
    let mut nb = NaiveBayes::new();

    let documents = prepare_documents(
      read_training_examples(training_path).unwrap()
    );

    for (category, tokens) in documents {
        nb.add_document(&tokens, &category);
    }

    nb.train();

    return Box::new(move |input_str| nb.classify(&tokenize(input_str)));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let classify = train(args.get(1).unwrap().to_owned());

    println!("{:#?}", classify("https://www.theguardian.com/sport/2019/feb/22/zion-williamson-injury-duke-nike-hypocrisy".to_owned()));
    println!("{:#?}", classify("https://www.nytimes.com/2019/02/21/world/asia/china-handwriting-robot.html".to_owned()));
    println!("{:#?}", classify("https://bgr.com/2019/02/20/moon-photo-50000-photos-andrew-mccarthy/".to_owned()));
}
