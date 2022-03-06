use crate::prelude::*;

pub fn filter_incorrect_letters(letters: Vec<char>) -> Vec<&'static str> {
    let mut found: Vec<&str> = ANSWER_LIST.to_vec();

    found.retain(|word| {
        for l in &letters {
            if word.to_lowercase().contains(&l.to_string().to_lowercase()) {
                return false;
            }
        }
        return true;
    });

    found
}

pub fn filter_correct_letters(letters: [char; 5], mut words: Vec<&str>) -> Vec<&str> {
    let mut empty = false;

    // Check for no correct placed letters. If so, return unchanged list
    for l in letters {
        if l == '\0' {
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

    words.retain(|word| {
        for (i, c) in word.char_indices() {
            if letters[i] == '\0' {
                continue;
            }

            if letters[i].to_string().to_lowercase() == c.to_string().to_lowercase() {
                continue;
            } else {
                return false;
            }
        }

        true
    });

    words
}

pub fn filter_found_letters(letters: &str, mut words_found: Vec<&'static str>) -> Vec<&'static str> {
    let mut found = false;
    if letters == "\0" || letters == "" {
        return words_found;
    } else {
        words_found.retain(|word| {
            for c in letters.to_lowercase().chars() {
                if word.contains(c) {
                    found = true;
                } else {
                    return false;
                }
            }
            found
        });

        words_found
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_get_words_letters_in_place() {
        // let incorrect: Vec<char> = vec!['t', 'a', 'd'];
        let incorrect: Vec<char> = vec![];
        let correct: [char; 5] = ['\0', 'R', '\0', '\0', 'e'];
        let found: &str = "B";

        let words = filter_incorrect_letters(incorrect);
        let words = filter_correct_letters(correct, words);
        let results = filter_found_letters(found, words);
        println!("results {}", results.len().to_string());
        println!("results {:?}", results);
        // assert_eq!(get_words_letters_in_placed(letters, incorrect).len(), 5);
    }
}
