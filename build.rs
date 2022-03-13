use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::net::Incoming;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize, Default)]
struct AnswerList {
    answers: Vec<WordleWord>,
}


#[derive(Debug, Deserialize, Serialize, Default)]
struct IncorrectList {
    incorrect: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct WordleWord {
    pub word: String,
    pub zipf_freq: f32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct IncorrectWord {
    pub word: String,
}

impl fmt::Display for WordleWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WordleWord {{ word: \"{}\", zipf_freq: {:.2}}}",
            self.word, self.zipf_freq as f32
        )
    }
}

impl fmt::Display for IncorrectWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IncorrectWord {{ word: \"{}\"}}", self.word)
    }
}

fn build_wordle_word_list(word_list: Vec<WordleWord>) -> String {
    let mut list: String = String::new();
    for w in word_list {
        list = list + &format!("\t{},\n", w);
    }
    list
}

fn build_incorrect_word_list(word_list: Vec<String>) -> String {
    let mut list: String = String::new();
    for w in word_list {
        list = list + &format!("\t\"{}\",\n", w);
    }
    list
}

fn main() {
    let answers = std::fs::File::open("list_utils/answers_scored.json").unwrap();
    // let file = std::fs::File::open("words.json").unwrap();
    let answers = std::io::BufReader::new(answers);
    let answer_list: AnswerList = serde_json::from_reader(answers).expect("error parsing json file");

    let incorrect = std::fs::File::open("list_utils/incorrect_from_nyt.json").unwrap();
    let incorrect = std::io::BufReader::new(incorrect);
    let incorrect_list: IncorrectList = serde_json::from_reader(incorrect).expect("error parsing json file");

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
        zipf_freq: 3.57
    }},
    WordleWord {{
        word: \"abase\",
        zipf_freq: 2.6
    }},
    WordleWord {{
        word: \"abate\",
        zipf_freq: 3.23
    }},
    WordleWord {{
        word: \"abbey\",
        zipf_freq: 2.18
    }},
    WordleWord {{
        word: \"abbot\",
        zipf_freq: 4.14
    }}
];

pub static ANSWER_LIST: [WordleWord; {}] = 
[
{}
];

pub static INCORRECT_LIST: [&str; {}] = 
[
{}
];
",
        answer_list.answers.len(),
        build_wordle_word_list(answer_list.answers),
        incorrect_list.incorrect.len(),
        build_incorrect_word_list(incorrect_list.incorrect)
    );

    fs::write(&dest_path, output).unwrap();
    println!("carg:rerun-if-changed=build.rs");
}
