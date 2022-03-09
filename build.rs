use serde::{Deserialize, Serialize};
// use std::env;
use std::path::Path;
use std::fmt;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct WordList {
    answers: Vec<WordleWord>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WordleWord {
    pub word: String,
    pub zipf_dist: f32,
}

impl fmt::Display for WordleWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WordleWord {{ word: \"{}\", zipf_dist: {}}}", self.word, self.zipf_dist)
    }
}

fn build_list(word_list: Vec<WordleWord>) -> String {
    let mut list: String = String::new();
    for w in word_list {
        list = list + &format!("\t{},\n", w);
    }
    list
}

fn main() {
    let file = std::fs::File::open("list_utils/words_test.json").unwrap();
    let file = std::io::BufReader::new(file);
    let word_lists: WordList = serde_json::from_reader(file).expect("error parsing json file");
    
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = "./src";
    let dest_path = Path::new(&out_dir).join("words.rs");
    let output: String = format!(
"
use crate::prelude::*;

#[allow(dead_code)]
pub static TEST_ANSWER_LIST: [WordleWord; 5] = 
[
    WordleWord {{
        word: \"aback\",
        zipf_dist: 3.57
    }},
    WordleWord {{
        word: \"abase\",
        zipf_dist: 2.6
    }},
    WordleWord {{
        word: \"abate\",
        zipf_dist: 3.23
    }},
    WordleWord {{
        word: \"abbey\",
        zipf_dist: 2.18
    }},
    WordleWord {{
        word: \"abbot\",
        zipf_dist: 4.14
    }}
];

pub static ANSWER_LIST: [WordleWord; {}] = 
[
{}
];", word_lists.answers.len(), build_list(word_lists.answers));
    
    fs::write(
        &dest_path, output   
    ).unwrap();
    println!("carg:rerun-if-changed=build.rs");
}