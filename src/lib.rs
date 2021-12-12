use rand::seq::SliceRandom;
use rand::thread_rng;
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
    pub take_first: Option<usize>,
    pub take_rand: Option<usize>,
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
    pub cut_to: Option<usize>,
}

/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>. (Blank
/// lines and duplicate links will be handled elsewhere.)
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

/// Like `make_vec_from_filenames`, this function takes a slice of `PathBuf`s of
/// files. But in this case these files represent lists of homophones that the
/// user wants to make sure aren't both on the resulting list.
///
/// These homophone files are expected to be formatted such that each line of the file
/// is `homophone1,homophone2`.
///
/// This function produces a Vector of tuples of strings, representing the
/// homophone pairs.
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

/// Simple helper functions that splits a `str` by a given substring `str`,
/// Then returns a Vector of `str`s.
/// ```
/// use tidy::split_and_vectorize;
/// assert_eq!(split_and_vectorize("a:b:c",":"), vec!["a","b","c"]);
/// ```
/// I find this a handy general helper function.
pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    string_to_split.split(splitter).collect()
}

/// This is the large, key function of the program. It takes
/// a `TidyRequest` object -- which includes the word list --
/// and performs whatever functions the user has requesteed to
/// perform on the list.
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
    tidied_list = match req.take_rand {
        Some(amount_to_take) => {
            let mut rng = thread_rng();
            tidied_list.shuffle(&mut rng);
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

    // Sort and dedup here
    tidied_list = sort_and_dedup(&mut tidied_list);

    // User can cut words from nearly finished list.
    // Does so randomly.
    tidied_list = match req.cut_to {
        Some(amount_to_cut) => {
            let mut rng = thread_rng();
            tidied_list.shuffle(&mut rng);
            tidied_list.truncate(amount_to_cut);
            tidied_list
        }
        None => tidied_list,
    };
    // Finally, sort and dedup list (for final time)
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list
}

/// Given a String (a word), delete all integers from the word.
fn delete_integers(mut word: String) -> String {
    word.retain(|c| !c.is_numeric());
    word
}

/// Given a String (a word), delete all characters that are not
/// alphanumeric
/// ```
/// use tidy::delete_nonalphanumeric;
/// assert_eq!(delete_nonalphanumeric("Hello!".to_string()), "Hello");
/// assert_eq!(delete_nonalphanumeric("world824...".to_string()), "world824");
/// ```
pub fn delete_nonalphanumeric(mut word: String) -> String {
    word.retain(|c| c.is_alphanumeric());
    word
}

/// Delete all character through and including the first appearance
/// of character `ch` in inputted `&str` `s`. Program uses this to
/// remove character through first tab or first space, a common task
/// when dealing with diceware passphrase word lists that have dice roll
/// numbers before each word. The
/// [EFF long list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)
/// is one such example.
///
/// Uses [memchr library](https://docs.rs/memchr/latest/memchr/)
/// to find this character a bit quicker than standard function.
///
/// I outlined other approaches to this function in
/// [a separate repo](https://github.com/sts10/splitter/blob/main/src/lib.rs).
fn delete_through_first_char(s: &str, ch: char) -> &str {
    match memchr(ch as u8, s.as_bytes()) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

use std::collections::HashMap;
/// This function removes words from the given word list
/// such that the resulting, outputted list has a guaranteed
/// maximum prefix length.  
///
/// As an example, if `maximum_shared_prefix_length == 4`, this
/// means that on the resulting list, knowing the first 4 characters
/// of any word on the generated list is sufficient to know which
/// word it is. In this case, we'd know that if a word starts with
/// "radi", we know it must be the word "radius" (if "radical" had been
/// on the list, this function would have removed it).
///
/// This is useful if you intend the list to be used by software that
/// uses auto-complete. In the case described above, a user will only have to type the
/// first 4 characters of any word before a program could successfully
/// auto-complete the entire word.
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

/// Given a word and a `usize` of `length`, this function returns
/// the first `length` characters of that word.
/// ```
/// use tidy::get_prefix;
/// assert_eq!(get_prefix("hello world", 4), "hell")
/// ```
pub fn get_prefix(word: &str, length: usize) -> String {
    word.chars().take(length).collect::<String>()
}

/// Iterates through a list of lines. Only `retain` lines
/// that are not empty (`!line.is_empty()`)
fn remove_blank_lines(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|line| !line.is_empty());
    new_list
}

/// Search inputted list for any words that have any
/// non-alphanumeric characters in it. Retain only words
/// that have only alphanumeric characters in it.
fn remove_nonalphanumeric(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|word| !word.chars().any(|c| !c.is_alphanumeric()));
    new_list
}

/// Search inputted list for any words that have any
/// non-alphabetic characters in them. Retain only words
/// that have only alphabetic characters in it.
fn remove_nonalphabetic(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|word| !word.chars().any(|chr| !is_alphabetic(chr as u8)));
    new_list
}

/// Helper function to determine if a given `u8` is a letter.
fn is_alphabetic(chr: u8) -> bool {
    (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
}

/// Search inputted list for any words that have any
/// integers in them (as determined by `is_numeric()`).
/// Retain only words that don't have any integers in them.
fn remove_integers(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|word| !word.chars().any(|c| c.is_numeric()));
    new_list
}

/// Iterates through list of `String`s, removing leading
/// and trailing whitespace from each `String`.
fn trim_whitespace(list: &[String]) -> Vec<String> {
    list.iter()
        .map(|w| w.trim_start().trim_end().to_string())
        .collect()
}

/// Alphabetizes and de-duplicates a Vector of `String`s.
///
/// For Rust's [`dedup()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup)
/// function to remove all duplicates, the Vector needs to be
/// [`sort()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)ed first.
fn sort_and_dedup(list: &mut Vec<String>) -> Vec<String> {
    list.sort();
    list.dedup();
    list.to_vec()
}

/// Remove prefix words from the given Vector of `String`s.
///
/// A brief example: If both "news" and "newspaper" are on the inputted list
/// we may, for cryptographic reasons, want to remove the prefix word,
/// which is "news" in this case.
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

/// Only retain words that are the given `minimum_edit_distance` away from all
/// other words on the list.
///
/// Calulates edit distance using a function in the edit_distance module.
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

/// Remove all words on the list that are on the given `reject_list`.
/// This `reject_list`, a Vector of `String`s, is read from a file.
fn remove_reject_words(list: Vec<String>, reject_list: Vec<String>) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| !reject_list.contains(x));
    new_list
}

/// Remove all words on the list that are NOT on the given `approved_list`
/// This `approved_list`, a Vector of `String`s, is read from a file.
fn remove_words_not_on_approved_list(list: Vec<String>, approved_list: Vec<String>) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| approved_list.contains(x));
    new_list
}

/// Remove words that are below the inputted `minimum_length`.
///
/// Any words equal to the inputted `minimum_length` will be preserved.
/// For example, if `minimum_length` is set to `4`, 4-character
/// words like "star" will be preserved.
fn remove_words_below_minimum_length(list: Vec<String>, minimum_length: usize) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|w| w.chars().count() >= minimum_length);
    new_list
}

/// Remove words that are above the inputted `maximum_length`.
///
/// Any words equal to the inputted `maximum_length` will be preserved.
/// For example, if `maximum_length` is set to `9`, 9-character
/// words like "alignment" will be preserved.
fn remove_words_above_maximum_length(list: Vec<String>, maximum_length: usize) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|w| w.chars().count() <= maximum_length);
    new_list
}

/// Takes the inputted word list and a Vector of tuples of Strings,
/// each representing a pair of homophones, e.g. `("there", "their")`.
/// The function outputs a new list in which, if both homophones
/// are detected, the second homophone is removed.
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

use radix_fmt::*; // https://stackoverflow.com/a/50278316
/// Print dice rolls before each corresponding word. Note
/// that the `n` parameter should be zero-indexed. A tab (`\t`)
/// is printed between the dice roll and the word.
///
/// The `base` parameter represents the number of sides of the
/// dice, which can be set from 2 to 9.
///
/// Here's an example of an outputted word list with base 6:
/// ```text
/// 11111	aback
/// 11112	abandons
/// 11113	abated
/// 11114	abbey
/// 11115	abbot
/// 11116	abbreviated
/// 11121	abdomen
/// 11122	abducted
/// 11123	aberrant
/// 11124	abide
/// 11125	ability
/// 11126	abject
/// 11131	abnormally
/// // etc.
/// ```
///
/// If this base is between 4 and 8,
/// this function assumes the user will be using actual dice, which are index at 1.
/// Thus, `if 4 <= base && base <= 8`, we add `1` to each digit of the dice
/// roll before printing it.
///
/// I wish I could replicate this radix function easily without the dependency,
/// but that doesn't seem [very easy](https://stackoverflow.com/a/50278316).
pub fn print_as_dice(n: usize, base: u8, list_length: usize) -> String {
    // Set width for zero-padding

    // First, get the literal width of the largest number we'll be printing.
    // This is, by definition the length of the list.
    // We want the length of the number in the base we want to print all
    // the numbers, so use radix function.
    let n_as_base = radix(n, base);
    let n_width = n_as_base.to_string().len();
    let pad_width = radix(list_length - 1, base).to_string().len();

    // Pad dice roll numbers with zeros
    // let padded_n = format!(
    //     "{:0width$}",
    //     n_as_base.to_string().parse::<usize>().unwrap(), // all this is needed
    //     width = pad_width
    // );
    let mut padded_n = "".to_string();
    for _i in n_width..pad_width {
        padded_n.push('0');
    }
    // Now that we have the appropriate number of zeros
    // in `padded_n`, it's time to add our number
    padded_n += &n_as_base.to_string();

    // If base is a common dice size (between 4 and 8),
    // we'll add one to each digit, to make it easier
    // to compare to actual rolled dice
    if 4 <= base && base <= 8 {
        padded_n
            .chars()
            .map(|ch| (ch.to_string().parse::<usize>().unwrap() + 1).to_string())
            .collect::<String>()

    // If base is over base 9, we'll add hyphens between digits to
    // make it easier to read.
    } else if base > 9 {
        padded_n
            .chars()
            .map(|ch| ch.to_string() + "-")
            .collect::<String>()[0..padded_n.chars().count() * 2 - 1]
            .trim()
            .to_string()
    } else {
        padded_n
    }
}

/// Calculate the entropy per word of a word list, given its size.
/// Entropy is meausred in bits, hence use of constant `2`.
///
/// Returns `f64` because this value to return (bits of entropy per
/// word) will most likely not be a whole bumber (which is fine!)
pub fn calc_entropy_per_word(list_size: usize) -> f64 {
    (list_size as f64).ln() / (2_f64.ln() as f64)
}
