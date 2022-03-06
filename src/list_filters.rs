use crate::{prelude::*, words};

pub fn get_words_letters_in_placed(letters: [char; 5], incorrect: Vec<char>) -> Vec<&'static str> {
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
        return answer_list.iter().map(|f| *f).collect::<Vec<&str>>();
    }

    let mut found: Vec<&str> = answer_list.to_vec();
    found.retain(|word| {
        for i in &incorrect {
            if word.to_lowercase().contains(&i.to_string().to_lowercase()) {
                return false;
            }
        }
        return true;
    });

    // for line in answer_list {
    //     for i in &incorrect {
    //         if line.contains(*i) {
    //             continue;
    //         } else {
    //             found.push(line);
    //         }
    //     }
    // }

    found.retain(|word| {
        let mut exists = false;

        for (i, c) in word.char_indices() {
            if letters[i] == '\0' {
                continue;
            }
            if letters[i].to_string().to_lowercase() == c.to_string().to_lowercase() {
                exists = true;
            } else {
                exists = false;
            }
        }

        exists
    });

    found
}

pub fn get_words_letters_contained(letters: &str, mut words_found: Vec<&'static str>) -> Vec<&'static str> {
    if letters == "\0" || letters == "" {
        return words_found;
    } else {
        words_found.retain(|word|{
            for c in letters.to_lowercase().chars() {
                if word.contains(c) {
                    return true;
                } else {
                    return false;
                }
            }
            false 
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
        let correct: [char; 5] = ['\0', 'R', '\0', '\0', '\0'];
        let found : &str = "";
    
        let words_found = get_words_letters_in_placed(correct, incorrect);
        let results = get_words_letters_contained(found, words_found);
        println!("results {}", results.len().to_string());
        // assert_eq!(get_words_letters_in_placed(letters, incorrect).len(), 5);
    }
}
