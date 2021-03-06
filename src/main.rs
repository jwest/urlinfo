#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate classifier;
extern crate regex;

use serde_json::Result;
use regex::Regex;
use std::fs::File;
use std::io::Read;

use classifier::NaiveBayes;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct TraningUrl {
    url: String,
    category: String,
    title: Option<String>,
}

fn read_training_examples() -> Result<Vec<TraningUrl>> {
    let mut file = File::open("traning-dataset.json").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read error");

    let data: Vec<TraningUrl> = serde_json::from_str(&contents).expect("json parse error");

    Ok(data)
}

fn tokenize(input: String) -> String {
    let re = Regex::new(r"[|\s,-_/?=&#:’`]+").expect("reg err");
    let mut fields: Vec<&str> = re.split(&input).collect();
    fields.retain(|&x| x != "http" && x != "https" && x != "www" && x != "com");
    return fields.join(" ");
}

fn train() -> NaiveBayes {
    let mut nb = NaiveBayes::new();

    let mut documents: HashMap<String, String> = HashMap::new();

    let examples = read_training_examples().unwrap();
    for example in examples {

      let title = example.title.unwrap_or(" ".to_string());
      let mut input = example.url;
      input.push_str(&" ".to_string());
      input.push_str(&title);
      let tokens = tokenize(input);

      if !documents.contains_key(&example.category) {
        documents.insert(example.category.to_owned(), "".to_owned());
      }

      let document_tokens = documents.get_mut(&example.category).unwrap();

      document_tokens.push_str(" ");
      document_tokens.push_str(&tokens);
    }

    for (category, tokens) in documents {
        nb.add_document(&tokens, &category);
    }

    nb.train();

    return nb;
}

fn main() {
    let nb = train();

    println!("{:#?}", nb.classify(&tokenize("https://www.theguardian.com/sport/2019/feb/22/zion-williamson-injury-duke-nike-hypocrisy".to_owned())));
    println!("{:#?}", nb.classify(&tokenize("https://www.nytimes.com/2019/02/21/world/asia/china-handwriting-robot.html".to_owned())));
    println!("{:#?}", nb.classify(&tokenize("https://bgr.com/2019/02/20/moon-photo-50000-photos-andrew-mccarthy/".to_owned())));
}
