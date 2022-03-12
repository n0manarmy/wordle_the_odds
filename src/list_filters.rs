use crate::prelude::*;

pub fn filter_incorrect_letters(letters: Vec<char>) -> Vec<WordleWord> {
    let mut found: Vec<WordleWord> = ANSWER_LIST.to_vec();

    found.retain(|wordle_word| {
        for l in &letters {
            if wordle_word
                .word
                .to_lowercase()
                .contains(&l.to_string().to_lowercase())
            {
                return false;
            }
        }
        return true;
    });

    found
}

pub fn filter_correct_letters(letters: [&str; 5], mut words: Vec<WordleWord>) -> Vec<WordleWord> {
    let mut empty = false;

    // Check for no correct placed letters. If so, return unchanged list
    for l in letters {
        if l == "" {
            empty = true;
        } else {
            empty = false;
            break;
        }
    }
    // No placed letters found, returning unchanged list
    if empty == true {
        return words;
    }

    words.retain(|wordle_word| {
        for (i, c) in wordle_word.word.char_indices() {
            if letters[i] == "" {
                continue;
            }

            if letters[i].to_lowercase() == c.to_string().to_lowercase() {
                continue;
            } else {
                return false;
            }
        }

        true
    });

    words
}

pub fn filter_found_letters(letters: &str, mut words_found: Vec<WordleWord>) -> Vec<WordleWord> {
    if letters == "" {
        return words_found;
    } else {
        words_found.retain(|wordle_words| {
            for c in letters.to_lowercase().chars() {
                if !wordle_words.word.contains(c) {
                    return false;
                }
            }
            true
        });
    }

    words_found
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_get_words_letters_in_place() {
        // let incorrect: Vec<char> = vec!['t', 'a', 'd'];
        let incorrect: Vec<char> = vec![];
        let correct: [&str; 5] = ["", "", "", "", "e"];
        let found: &str = "TODAY";

        let wordle_words = filter_incorrect_letters(incorrect);
        dbg!(&wordle_words.len());
        let wordle_words = filter_correct_letters(correct, wordle_words);
        dbg!(&wordle_words.len());
        let results = filter_found_letters(found, wordle_words);
        println!("results {}", results.len().to_string());
        println!("results {:?}", results);
        // assert_eq!(get_words_letters_in_placed(letters, incorrect).len(), 5);
    }

    #[test]
    pub fn test_get_letters_found() {
        // let incorrect: Vec<char> = vec!['t', 'a', 'd'];

        const TEST_ANSWER_LIST_FOUND: [WordleWord; 5] = [
            WordleWord {
                word: "abcde",
                zipf_freq: 3.57,
            },
            WordleWord {
                word: "bcdea",
                zipf_freq: 2.6,
            },
            WordleWord {
                word: "cdeab",
                zipf_freq: 3.23,
            },
            WordleWord {
                word: "deabc",
                zipf_freq: 2.18,
            },
            WordleWord {
                word: "eabcd",
                zipf_freq: 4.14,
            },
        ];
        let wordle_words: Vec<WordleWord> = ANSWER_LIST.to_vec();
        let found: &str = "";
        let results = filter_found_letters(found, wordle_words);
        assert_eq!(results.len(), 2309);

        let found: &str = "TODAY";
        let wordle_words: Vec<WordleWord> = ANSWER_LIST.to_vec();
        let results = filter_found_letters(found, wordle_words);
        // assert_eq!(results.len(), 5);

        println!("results {}", results.len().to_string());
        println!("results {:?}", results);
        // assert_eq!(get_words_letters_in_placed(letters, incorrect).len(), 5);
    }
}
