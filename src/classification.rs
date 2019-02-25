use classifier::NaiveBayes;

use crate::tokenization::{tokenize};
use crate::training::{prepare_documents, read_training_examples};

pub fn classifier(training_path: String) -> Box<Fn(String) -> String> {
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