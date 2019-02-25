use serde_json::Result;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use crate::domain::{TraningUrl};

pub fn read_training_examples(training_path: String) -> Result<Vec<TraningUrl>> {
    let mut file = File::open(training_path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read error");

    let data: Vec<TraningUrl> = serde_json::from_str(&contents).expect("json parse error");

    Ok(data)
}

pub fn prepare_documents(examples: Vec<TraningUrl>) -> HashMap<String, String> {
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