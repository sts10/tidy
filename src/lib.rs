use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use memchr::memchr;

#[derive(Default, Debug, Clone)]
pub struct TidyRequest {
    pub list: Vec<String>,
    pub to_lowercase: bool,
    pub should_remove_prefix_words: bool,
    pub should_remove_nonalphanumeric: bool,
    pub should_delete_nonalphanumeric: bool,
    pub should_remove_integers: bool,
    pub should_delete_integers: bool,
    pub should_delete_through_first_tab: bool,
    pub should_delete_through_first_space: bool,
    pub reject_list: Option<Vec<String>>,
    pub approved_list: Option<Vec<String>>,
    pub homophones_list: Option<Vec<(String, String)>>,
    pub minimum_length: Option<usize>,
    pub maximum_length: Option<usize>,
    pub unique_prefix_length: Option<usize>,
}

pub fn make_vec_from_filenames(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            word_list.push(l);
        }
    }
    word_list
}

pub fn read_homophones_list_from_filenames(filenames: &[PathBuf]) -> Vec<(String, String)> {
    let mut homophones_list: Vec<(String, String)> = vec![];
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            let pair: (String, String) = (
                split_and_vectorize(&l, ",")[0].trim().to_string(),
                split_and_vectorize(&l, ",")[1].trim().to_string(),
            );
            homophones_list.push(pair);
        }
    }
    homophones_list
}

fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    string_to_split.split(splitter).collect()
}

pub fn tidy_list(req: TidyRequest) -> Vec<String> {
    // guess this function is what I should clean-up next...
    let mut tidied_list = if req.should_delete_through_first_tab {
        req.list
            .iter()
            // Should figure out how to remove these to_string calls
            .map(|w| delete_through_first_char(w, '\t').to_string())
            .collect()
    } else {
        req.list
    };
    tidied_list = if req.should_delete_through_first_space {
        tidied_list
            .iter()
            .map(|w| delete_through_first_char(w, ' ').to_string())
            .collect()
    } else {
        tidied_list
    };
    tidied_list = if req.should_delete_integers {
        tidied_list
            .iter()
            .map(|w| delete_integers(w.to_string()))
            .collect()
    } else {
        tidied_list
    };
    tidied_list = if req.should_delete_nonalphanumeric {
        tidied_list
            .iter()
            .map(|w| delete_nonalphanumeric(w.to_string()))
            .collect()
    } else {
        tidied_list
    };
    tidied_list = trim_whitespace(&tidied_list);
    tidied_list = remove_blank_lines(&tidied_list);
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list = if req.to_lowercase {
        tidied_list.iter().map(|w| w.to_ascii_lowercase()).collect()
    } else {
        tidied_list
    };
    tidied_list = if req.should_remove_nonalphanumeric {
        remove_nonalphanumeric(&tidied_list)
    } else {
        tidied_list
    };
    tidied_list = if req.should_remove_integers {
        remove_integers(&tidied_list)
    } else {
        tidied_list
    };
    tidied_list = match req.reject_list {
        Some(reject_list) => remove_reject_words(tidied_list, reject_list),
        None => tidied_list,
    };
    tidied_list = match req.approved_list {
        Some(approved_list) => remove_words_not_on_approved_list(tidied_list, approved_list),
        None => tidied_list,
    };
    tidied_list = match req.homophones_list {
        Some(homophones_list) => remove_homophones(tidied_list, homophones_list),
        None => tidied_list,
    };
    tidied_list = match req.minimum_length {
        Some(minimum_length) => remove_words_below_minimum_length(tidied_list, minimum_length),
        None => tidied_list,
    };
    tidied_list = match req.maximum_length {
        Some(maximum_length) => remove_words_above_maximum_length(tidied_list, maximum_length),
        None => tidied_list,
    };
    tidied_list = match req.unique_prefix_length {
        Some(unique_prefix_length) => {
            guarantee_unique_prefix_length(&tidied_list, unique_prefix_length)
        }
        None => tidied_list,
    };
    tidied_list = if req.should_remove_prefix_words {
        remove_prefix_words(sort_and_dedup(&mut tidied_list))
    } else {
        tidied_list
    };
    // Have to remove any and all blank lines again in case any of
    // the delete calls made new blank lines
    tidied_list = remove_blank_lines(&tidied_list);
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list
}

fn delete_integers(mut w: String) -> String {
    w.retain(|c| !c.is_numeric());
    w
}

fn delete_nonalphanumeric(mut w: String) -> String {
    w.retain(|c| c.is_alphanumeric());
    w
}

// Use memchr library to find ch, then split string at that position
// Other approaches to this function: https://github.com/sts10/splitter/blob/main/src/lib.rs
fn delete_through_first_char(s: &str, ch: char) -> &str {
    match memchr(ch as u8, s.as_bytes()) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

use std::collections::HashMap;
fn guarantee_unique_prefix_length(list: &[String], unique_prefix_length: usize) -> Vec<String> {
    let mut prefix_hashmap: HashMap<String, String> = HashMap::new();
    for this_word in list {
        // If this word is too short just skip it.
        if this_word.len() < unique_prefix_length {
            continue;
        }
        prefix_hashmap
            .entry(get_prefix(this_word, unique_prefix_length))
            .and_modify(|existing_word| {
                if this_word.len() < existing_word.len() {
                    *existing_word = this_word.to_string()
                }
            })
            // .or_insert(this_word.to_string());
            .or_insert_with(|| this_word.to_string());
    }
    let new_vec: Vec<(&String, &String)> = prefix_hashmap.iter().collect();
    let mut new_word_list = vec![];
    for t in new_vec {
        new_word_list.push(t.1.to_string());
    }
    new_word_list
}

fn get_prefix(word: &str, length: usize) -> String {
    word[0..length].to_string()
}

fn remove_blank_lines(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| !x.is_empty());
    new_list
}

fn remove_nonalphanumeric(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    // This logic trips me out a bit, but I think it
    // works correctly
    new_list.retain(|word| !word.chars().any(|c| !c.is_alphanumeric()));
    new_list
}

fn remove_integers(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    // This logic trips me out a bit, but I think it
    // works correctly
    new_list.retain(|word| !word.chars().any(|c| c.is_numeric()));
    new_list
}

fn trim_whitespace(list: &[String]) -> Vec<String> {
    list.iter()
        .map(|w| w.trim_start().trim_end().to_string())
        .collect()
}

fn sort_and_dedup(list: &mut Vec<String>) -> Vec<String> {
    list.sort();
    list.dedup();
    list.to_vec()
}

fn remove_prefix_words(list: Vec<String>) -> Vec<String> {
    let mut list_without_prefix_words = list.to_vec();
    list_without_prefix_words.retain(|potential_prefix_word| {
        for word in &list {
            if word.starts_with(potential_prefix_word) && word != potential_prefix_word {
                // This is a prefix word, so we do NOT want to retain it. return false to the
                // retain
                return false;
            } else {
                // This particular word is not a prefix word of this potential_prefix_word.
                // keep looping
                continue;
            };
        }
        // If we've made it here, we can be sure that potential_prefix_word is NOT a
        // prefix word. So we want to retain it for the list_without_prefix_words.
        // To do this, we return true to the retain.
        true
    });
    list_without_prefix_words
}

fn remove_reject_words(list: Vec<String>, reject_list: Vec<String>) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| !reject_list.contains(x));
    new_list
}

fn remove_words_not_on_approved_list(list: Vec<String>, approved_list: Vec<String>) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| approved_list.contains(x));
    new_list
}

fn remove_words_below_minimum_length(list: Vec<String>, minimum_length: usize) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|w| w.len() >= minimum_length);
    new_list
}

fn remove_words_above_maximum_length(list: Vec<String>, maximum_length: usize) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|w| w.len() <= maximum_length);
    new_list
}

fn remove_homophones(list: Vec<String>, homophones: Vec<(String, String)>) -> Vec<String> {
    let mut words_to_remove = vec![];
    for pair_of_homophones in homophones {
        if list.contains(&pair_of_homophones.0)
            && list.contains(&pair_of_homophones.1)
            && !(words_to_remove.contains(&pair_of_homophones.0)
                || words_to_remove.contains(&pair_of_homophones.1))
        {
            words_to_remove.push(pair_of_homophones.1);
        }
    }
    let mut new_list = list.to_vec();
    new_list.retain(|w| !words_to_remove.contains(w));
    new_list
}

// I'm pretty sure this is an accurate, if obtuse method of calculating entropy
// of a word list, given its size
pub fn calc_entropy(list_size: usize) -> f64 {
    (list_size as f64).ln() / (2_f64.ln() as f64)
}

pub fn is_below_brute_force_line(list: &[String]) -> bool {
    let list_length = list.len() as f64;
    let shortest_word_length = get_shortest_word_length(list);
    (shortest_word_length as f64) < list_length.log(26.0)
}

pub fn get_shortest_word_length(list: &[String]) -> usize {
    let mut shortest_word_length: usize = usize::max_value();
    for word in list {
        if word.len() < shortest_word_length {
            shortest_word_length = word.len();
        }
    }
    shortest_word_length
}
