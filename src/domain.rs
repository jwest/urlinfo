use serde_derive::{Deserialize, Serialize};

use crate::tokenization::{tokenize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TraningUrl {
    pub url: String,
    pub category: String,
    pub title: Option<String>,
}

impl TraningUrl {
  pub fn get_tokens(&self) -> String {
    let title = self.title.clone().unwrap_or(" ".to_string());
    let mut input = self.url.clone();
    input.push_str(" ");
    input.push_str(&title);
    return tokenize(input);
  }
}
