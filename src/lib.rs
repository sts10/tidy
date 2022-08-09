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
    pub sort_alphabetically: bool,
    pub ignore_from_delimiter: Option<char>,
    pub ignore_through_delimiter: Option<char>,
    pub to_lowercase: bool,
    pub should_straighten_quotes: bool,
    pub should_remove_prefix_words: bool,
    pub should_remove_suffix_words: bool,
    pub should_remove_nonalphanumeric: bool,
    pub should_delete_nonalphanumeric: bool,
    pub should_remove_nonalphabetic: bool,
    pub should_remove_non_latin_alphabetic: bool,
    pub should_remove_nonascii: bool,
    pub should_remove_integers: bool,
    pub should_delete_integers: bool,
    pub should_delete_from_first_delimiter: Option<char>,
    pub should_delete_through_first_delimiter: Option<char>,
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

/// Simple helper function that splits a `str` by a given substring `str`,
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
    let mut tidied_list = vec![];
    for word in &req.list {
        // haha, love this
        // If user chose to ignore metadata, split word vs. metadata on the first comma
        // found.
        // We'll then do removals operations on the "word", ignoriong metadata.
        // Later, we'll re-add the metadata to the word.

        // We need delimiter_to_use to have a broad scope so that we can use it
        // when we re-add the metadata at the end. Default to comma, but can be changed
        // in match statement here.
        let (mut new_word, delimiter, metadata, metadata_position) =
            match (req.ignore_from_delimiter, req.ignore_through_delimiter) {
                (Some(delimiter), None) => {
                    let split_vec = split_and_vectorize(word, &delimiter.to_string());
                    if split_vec.len() == 1 {
                        eprintln!("No metadata found for word: {:?}", word);
                        (word.to_string(), Some(delimiter), None, None)
                    } else {
                        (
                            split_vec[0].to_string(),
                            Some(delimiter),
                            Some(split_vec[1]),
                            Some("end"), // this should be an enum!
                        )
                    }
                }
                (None, Some(delimiter)) => {
                    let split_vec = split_and_vectorize(word, &delimiter.to_string());
                    if split_vec.len() == 1 {
                        eprintln!("No metadata found for word: {:?}", word);
                        (word.to_string(), Some(delimiter), None, None)
                    } else {
                        (
                            split_vec[1].to_string(),
                            Some(delimiter),
                            Some(split_vec[0]),
                            Some("start"),
                        )
                    }
                }
                (Some(ref _delimiter1), Some(ref _delimiter2)) => {
                    // This situation should be caught and handled better
                    // in src/main.rs, so this is really just in case.
                    panic!("Can't ignore metadata on both sides currently")
                }
                (None, None) => (word.to_string(), None, None, None),
            };

        // Important to trim starting and ending whitespace first.
        new_word = new_word.trim_start().trim_end().to_string();

        // First, remove words that should be removed
        // Could switch to an Option?
        // let new_word: Option<String> = Some(new_word);

        if req.should_remove_nonascii {
            // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii
            if new_word.chars().any(|chr| !chr.is_ascii()) {
                new_word = "".to_string();
                // new_word = None
            }
        }
        if req.should_remove_nonalphanumeric {
            if new_word.chars().any(|c| !c.is_alphanumeric()) {
                new_word = "".to_string();
            }
        }
        if req.should_remove_nonalphabetic {
            if new_word.chars().any(|c| !c.is_alphabetic()) {
                new_word = "".to_string();
            }
        }
        if req.should_remove_non_latin_alphabetic {
            if new_word.chars().any(|chr| !is_latin_alphabetic(chr as u16)) {
                new_word = "".to_string();
            }
        }
        if req.should_remove_integers {
            if new_word.chars().any(|c| c.is_numeric()) {
                new_word = "".to_string();
            }
        }

        match req.reject_list {
            Some(ref reject_list) => {
                if reject_list.contains(&new_word) {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        match req.approved_list {
            Some(ref approved_list) => {
                if !approved_list.contains(&new_word) {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();

        match req.minimum_length {
            Some(minimum_length) => {
                if new_word.chars().count() < minimum_length {
                    new_word = "".to_string();
                }
            }
            None => (),
        };
        match req.maximum_length {
            Some(maximum_length) => {
                if new_word.chars().count() > maximum_length {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();

        // If there was metadata, re-add it to the word now.
        if new_word != "" {
            match metadata {
                Some(metadata) => {
                    if metadata_position == Some("end") {
                        new_word = new_word + &delimiter.unwrap().to_string() + metadata;
                    } else if metadata_position == Some("start") {
                        new_word =
                            metadata.to_owned() + &delimiter.unwrap().to_string() + &new_word;
                    }
                }
                None => (),
            };
        }

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();

        // Now on to word MODIFICATIONS, rather than word removals
        new_word = match req.should_delete_from_first_delimiter {
            Some(delimiter) => delete_after_first_char(&new_word, delimiter).to_string(),
            None => new_word,
        };
        new_word = match req.should_delete_through_first_delimiter {
            Some(delimiter) => delete_through_first_char(&new_word, delimiter).to_string(),
            None => new_word,
        };
        if req.should_delete_integers {
            new_word = delete_integers(new_word.to_string());
        }
        if req.should_delete_nonalphanumeric {
            new_word = delete_nonalphanumeric(new_word.to_string());
        }
        if req.to_lowercase {
            new_word = new_word.to_ascii_lowercase();
        }
        if req.should_straighten_quotes {
            new_word = straighten_quotes(&new_word).to_string();
        }

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();
        // check if blank
        // if new_word.is_some() && new_word != Some("") {
        if new_word != "" {
            tidied_list.push(new_word);
        }
    }
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
    // Some operations are just a bit too complex for
    // me to figure out how to do on a per-word basis
    // at this time. Maybe something to revisit in the
    // future
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
    tidied_list = if req.should_remove_suffix_words {
        remove_suffix_words(dedup_without_sorting(&mut tidied_list))
    } else {
        tidied_list
    };

    tidied_list = if req.should_remove_prefix_words {
        remove_prefix_words(dedup_without_sorting(&mut tidied_list))
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
    if req.sort_alphabetically {
        tidied_list.sort();
    }
    tidied_list = dedup_without_sorting(&mut tidied_list);

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
    // Finally, sort and dedup list one more time
    // (probably unneccesary, since we've only cut
    // words since that last time we sorted and de-duped.
    if req.sort_alphabetically {
        tidied_list.sort();
    }
    tidied_list = dedup_without_sorting(&mut tidied_list);
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

/// Delete all characters through and including the first appearance
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

/// Delete all characters after and including the first appearance
/// of character `ch` in inputted `&str` `s`.
///
/// Uses [memchr library](https://docs.rs/memchr/latest/memchr/)
/// to find this character a bit quicker than standard function.
///
/// I outlined other approaches to this function in
/// [a separate repo](https://github.com/sts10/splitter/blob/main/src/lib.rs).
fn delete_after_first_char(s: &str, ch: char) -> &str {
    match memchr(ch as u8, s.as_bytes()) {
        None => s, // not found => return the whole string
        Some(pos) => &s[0..pos],
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

/// Helper function to determine if a given char as `u16` is a
/// Latin letter (A through Z or a through z, no diacritics).
/// ```
/// use tidy::is_latin_alphabetic;
/// assert_eq!(is_latin_alphabetic('h' as u16), true);
/// assert_eq!(is_latin_alphabetic('A' as u16), true);
/// assert_eq!(is_latin_alphabetic('1' as u16), false);
/// assert_eq!(is_latin_alphabetic(',' as u16), false);
/// assert_eq!(is_latin_alphabetic('é' as u16), false);
/// assert_eq!(is_latin_alphabetic('ő' as u16), false);
/// ```
pub fn is_latin_alphabetic(chr: u16) -> bool {
    (chr >= 65 && chr <= 90) || (chr >= 97 && chr <= 122)
}

/// Replaces curly or smart quotes with straight quotes.
fn straighten_quotes(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '\u{201C}' => result.push('\"'), // LEFT DOUBLE QUOTATION MARK
            '\u{201D}' => result.push('\"'), // RIGHT DOUBLE QUOTATION MARK
            '\u{2018}' => result.push('\''), // LEFT SINGLE QUOTATION MARK
            '\u{2019}' => result.push('\''), // RIGHT SINGLE QUOTATION MARK
            _ => result.push(c),
        }
    }
    result
}

use itertools::Itertools;
/// De-duplicates a Vector of `String`s while maintaining list order.
fn dedup_without_sorting(list: &mut [String]) -> Vec<String> {
    let dedup: Vec<String> = list.iter().unique().map(|s| s.to_string()).collect();
    dedup.to_vec()
}

/// Remove prefix words from the given Vector of `String`s.
///
/// A brief example: If both "news" and "newspaper" are on the inputted list
/// we may, for security reasons, want to remove the prefix word,
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

/// Remove suffix words from the given Vector of `String`s.
///
/// A brief example: If both "news" and "newspaper" are on the inputted list
/// we may, for security reasons, want to remove the suffix word,
/// which is "paper" in this case.
fn remove_suffix_words(list: Vec<String>) -> Vec<String> {
    let mut list_without_suffix_words = list.to_vec();
    list_without_suffix_words.retain(|potential_suffix_word| {
        for word in &list {
            if word.ends_with(potential_suffix_word) && word != potential_suffix_word {
                // This is a suffix word, so we do NOT want to retain it. return false to the
                // retain
                return false;
            } else {
                // This particular word is not a suffix word of this potential_suffix_word.
                // keep looping
                continue;
            };
        }
        // If we've made it here, we can be sure that potential_suffix_word is NOT a
        // suffix word. So we want to retain it for the list_without_suffix_words.
        // To do this, we return true to the retain.
        true
    });
    list_without_suffix_words
}

/// Only retain words that are the given `minimum_edit_distance` away from all
/// other words on the list.
///
/// Calulates edit distance using a function in the edit_distance module.
fn enfore_minimum_edit_distance(list: Vec<String>, minimum_edit_distance: usize) -> Vec<String> {
    let minimum_edit_distance: u32 = minimum_edit_distance.try_into().unwrap();
    let mut list_to_read = list.to_vec();
    // Sort short words first to prefer them
    list_to_read.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));

    let mut new_list = list.to_vec();
    new_list.retain(|potential_too_close_word| {
        for word in &list_to_read {
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
                // This particular word is not too close to this potential_too_close_word.
                // keep looping
                continue;
            };
        }
        // If we've made it here, we can be sure that potential_too_close_word is NOT too
        // close to another  word. So we want to retain it for the new_list.
        // To do this, we return true to the retain.
        true
    });
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
/// 11111    aback
/// 11112    abandons
/// 11113    abated
/// 11114    abbey
/// 11115    abbot
/// 11116    abbreviated
/// 11121    abdomen
/// 11122    abducted
/// 11123    aberrant
/// 11124    abide
/// 11125    ability
/// 11126    abject
/// 11131    abnormally
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
pub fn print_as_dice(n: usize, base: u8, list_length: usize, use_letters: bool) -> String {
    // Set width for zero-padding

    // First, get the literal width of the largest number we'll be printing.
    // This is, by definition the length of the list.
    // We want the length of the number in the base we want to print all
    // the numbers, so use radix function.
    let n_as_base = radix(n, base);

    // Pad dice roll numbers with zeros
    let n_width = n_as_base.to_string().len();
    let pad_width = radix(list_length - 1, base).to_string().len();

    let mut padded_n = String::new();
    for _i in n_width..pad_width {
        padded_n.push('0');
    }
    // Now that we have the appropriate number of zeros
    // in `padded_n`, it's time to add our number
    padded_n += &n_as_base.to_string();

    // Print the dice rolls in slightly different ways,
    // depending on the value of the base.
    match base {
        // Values of 0 and 1 should have been caught earlier,
        // so we'll panic! if we have them here
        0 | 1 => panic!("Too few dice sides entered"),
        // If base is 2 or 3, just print as-is, zero-indexed.
        2 | 3 => padded_n,
        // If base is a common dice size (between 4 and 8), we'll add
        // one to each digit (i.e. no longer zero-indexed), to make it
        // easier to compare to actual rolled dice
        4..=8 => padded_n
            .chars()
            .map(|ch| (ch.to_string().parse::<usize>().unwrap() + 1).to_string())
            .collect::<String>(),
        // If base is over base 9, we'll print each digit as zero-indexed,
        // but we'll add a hyphen _between_ digits to make it easier to read.
        9..=36 => padded_n
            .chars()
            .map(|ch| format_high_dice_roll(ch, use_letters) + "-")
            .collect::<String>() //[0..padded_n.chars().count() * 2 - 1]
            .trim_end_matches('-')
            .trim()
            .to_string(),
        _ => panic!("Amount of dice sides received is too high"),
    }
}

fn format_high_dice_roll(ch: char, use_letters: bool) -> String {
    if !use_letters {
        match ch {
            '0' => "01",
            '1' => "02",
            '2' => "03",
            '3' => "04",
            '4' => "05",
            '5' => "06",
            '6' => "07",
            '7' => "08",
            '8' => "09",
            '9' => "10",
            'a' => "11",
            'b' => "12",
            'c' => "13",
            'd' => "14",
            'e' => "15",
            'f' => "16",
            'g' => "17",
            'h' => "18",
            'i' => "19",
            'j' => "20",
            'k' => "21",
            'l' => "22",
            'm' => "23",
            'n' => "24",
            'o' => "25",
            'p' => "26",
            'q' => "27",
            'r' => "28",
            's' => "29",
            't' => "30",
            'u' => "31",
            'v' => "32",
            'w' => "33",
            'x' => "34",
            'y' => "35",
            'z' => "36",
            _ => panic!("Unable to convert this dice number from a letter to a number."),
        }
        .to_string()
    } else {
        ch.to_string()
    }
}
