use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
pub mod display_information;
pub mod edit_distance;
use crate::edit_distance::find_edit_distance;

use memchr::memchr;

#[derive(Default, Debug, Clone)]
pub struct TidyRequest {
    pub list: Vec<String>,
    pub to_lowercase: bool,
    pub should_remove_prefix_words: bool,
    pub should_remove_nonalphabetic: bool,
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
    pub maximum_shared_prefix_length: Option<usize>,
    pub minimum_edit_distance: Option<usize>,
    pub take_first: Option<usize>,
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
    // Remove blank lines first
    let mut tidied_list = remove_blank_lines(&req.list);
    tidied_list = if req.should_delete_through_first_tab {
        tidied_list
            .iter()
            // Should figure out how to remove these to_string calls
            .map(|w| delete_through_first_char(w, '\t').to_string())
            .collect()
    } else {
        tidied_list
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
    // We're going to check for blank lines again in
    // case we made any lines blank with delete/trim calls
    tidied_list = remove_blank_lines(&tidied_list);
    // Now truncate list, if requested
    // Maybe should move this up in the order though...
    tidied_list = match req.take_first {
        Some(amount_to_take) => {
            tidied_list.truncate(amount_to_take);
            tidied_list
        }
        None => tidied_list,
    };
    tidied_list = if req.to_lowercase {
        tidied_list.iter().map(|w| w.to_ascii_lowercase()).collect()
    } else {
        tidied_list
    };
    // Move on to word removals
    tidied_list = if req.should_remove_nonalphanumeric {
        remove_nonalphanumeric(&tidied_list)
    } else {
        tidied_list
    };
    tidied_list = if req.should_remove_nonalphabetic {
        remove_nonalphabetic(&tidied_list)
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
    tidied_list = match req.minimum_length {
        Some(minimum_length) => remove_words_below_minimum_length(tidied_list, minimum_length),
        None => tidied_list,
    };
    tidied_list = match req.maximum_length {
        Some(maximum_length) => remove_words_above_maximum_length(tidied_list, maximum_length),
        None => tidied_list,
    };
    tidied_list = match req.homophones_list {
        Some(homophones_list) => remove_homophones(tidied_list, homophones_list),
        None => tidied_list,
    };
    // I think this is a good order for these next 3 operations,
    // but I'm not super confident
    tidied_list = match req.minimum_edit_distance {
        Some(minimum_edit_distance) => {
            enfore_minimum_edit_distance(tidied_list, minimum_edit_distance)
        }
        None => tidied_list,
    };
    tidied_list = if req.should_remove_prefix_words {
        remove_prefix_words(sort_and_dedup(&mut tidied_list))
    } else {
        tidied_list
    };
    tidied_list = match req.maximum_shared_prefix_length {
        Some(maximum_shared_prefix_length) => {
            guarantee_maximum_prefix_length(&tidied_list, maximum_shared_prefix_length)
        }
        None => tidied_list,
    };
    // Finally, sort and dedup list
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
// I could definitely understand an argument that this logic
// is off by one in a wonky way that I can't quite
// figure out.
fn guarantee_maximum_prefix_length(
    list: &[String],
    maximum_shared_prefix_length: usize,
) -> Vec<String> {
    let mut prefix_hashmap: HashMap<String, String> = HashMap::new();
    for this_word in list {
        // If this word is too short just skip it.
        if this_word.chars().count() < maximum_shared_prefix_length {
            continue;
        }
        prefix_hashmap
            .entry(get_prefix(this_word, maximum_shared_prefix_length))
            .and_modify(|existing_word| {
                // Prefer shorter words, as a stand-in for simplicity (though that
                // is debatable...)
                if this_word.chars().count() < existing_word.chars().count() {
                    *existing_word = this_word.to_string()
                }
            })
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
    word.chars().take(length).collect::<String>()
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

fn remove_nonalphabetic(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|word| !word.chars().any(|chr| !is_alphabetic(chr as u8)));
    new_list
}

fn is_alphabetic(chr: u8) -> bool {
    (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
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

fn enfore_minimum_edit_distance(list: Vec<String>, minimum_edit_distance: usize) -> Vec<String> {
    let minimum_edit_distance: u32 = minimum_edit_distance.try_into().unwrap();
    let mut new_list = list.to_vec();
    new_list.retain(|potential_too_close_word| {
        for word in &list {
            // Skip if we're looking at the same word
            if word == potential_too_close_word {
                continue;
            }
            if find_edit_distance(word, potential_too_close_word) < minimum_edit_distance {
                // This potential_too_close_word is too close to another word on the list,
                // so we do NOT want to retain it.
                // return false to the retain
                return false;
            } else {
                // This particular word is not too close to this potential_prefix_word.
                // keep looping
                continue;
            };
        }
        // If we've made it here, we can be sure that potential_prefix_word is NOT too
        // close to another  word. So we want to retain it for the new_list.
        // To do this, we return true to the retain.
        true
    });
    new_list
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
    new_list.retain(|w| w.chars().count() >= minimum_length);
    new_list
}

fn remove_words_above_maximum_length(list: Vec<String>, maximum_length: usize) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|w| w.chars().count() <= maximum_length);
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

use radix_fmt::*;
pub fn print_as_dice(n: usize, base: u8, list_length: usize) -> String {
    // Set width for zero-padding. Not 100% sure if this - 1 is correct.
    let pad_width = radix(list_length, base).to_string().len() - 1;
    let as_base = radix(n, base);

    // Pad dice roll numbers with zeros
    let padded = format!(
        "{:0width$}",
        as_base.to_string().parse::<usize>().unwrap(),
        width = pad_width
    );
    // If base is a common dice size under 10, we'll add one to
    // each digit, to make it easier to compare to rolled dice
    // if base >= 4 && base <= 8 {
    if (4..=8).contains(&base) {
        padded
            .chars()
            .map(|s| (s.to_string().parse::<usize>().unwrap() + 1).to_string())
            .collect::<String>()
    } else {
        padded
    }
}

// I'm pretty sure this is an accurate, if obtuse method of calculating entropy
// of a word list, given its size
pub fn calc_entropy(list_size: usize) -> f64 {
    (list_size as f64).ln() / (2_f64.ln() as f64)
}
