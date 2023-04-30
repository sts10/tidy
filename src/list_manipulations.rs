use crate::count_characters;
use crate::edit_distance::find_edit_distance;
use crate::sardinas_patterson_pruning::get_sardinas_patterson_final_intersection;
use memchr::memchr;
use unicode_normalization::UnicodeNormalization;

/// Normalize the Unicode of a string
/// See https://docs.rs/unicode-normalization/latest/unicode_normalization/trait.UnicodeNormalization.html#tymethod.nfc
pub fn normalize_unicode(word: &str, nf: &str) -> Result<String, String> {
    match nf.to_lowercase().as_str() {
        "nfc" => Ok(word.nfc().collect()),
        "nfd" => Ok(word.nfd().collect()),
        "nfkc" => Ok(word.nfkc().collect()),
        "nfkd" => Ok(word.nfkd().collect()),
        _ => Err("Unknown Unicode Normalization Form received in arguments.\nPlease use one of the following normalization forms: nfc, nfd, nfkc, or nfkd.".to_string()),
    }
}

use icu::collator::*;
use icu::locid::Locale;
use icu_collator::Collator;
use icu_collator::CollatorOptions;
/// Sort a Vector of words a bit more carefully than Rust's
/// default .sort(), treating capitalized letters and accented letters a
/// bit more smart.
/// `.sorted()` words -> ["Zambia", "abbey", "eager", "enlever", "ezra", "zoo", "énigme"]
/// sort_carefully words -> ["abbey", "eager", "énigme", "enlever", "ezra", "Zambia", "zoo"]
pub fn sort_carefully(list: Vec<String>, locale: Locale) -> Vec<String> {
    // let given_locale: Locale = match given_locale {
    //     Some(given_locale) => locale!(given_locale),
    //     None => locale!("en"),
    // };
    // let given_locale = locale!("en");
    let mut options_l2 = CollatorOptions::new();
    options_l2.strength = Some(Strength::Secondary);
    let collator_l2: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &locale.into(), options_l2).unwrap();
    let mut newly_sorted_list = list;
    newly_sorted_list.sort_by(|a, b| collator_l2.compare(a, b));
    newly_sorted_list
}

/// Given a String (a word), delete all integers from the word.
pub fn delete_integers(mut word: String) -> String {
    word.retain(|c| !c.is_numeric());
    word
}

/// Given a String (a word), delete all characters that are not
/// alphanumeric
/// ```
/// use tidy::list_manipulations::delete_nonalphanumeric;
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
pub fn delete_before_first_char(s: &str, ch: char) -> &str {
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
pub fn delete_after_first_char(s: &str, ch: char) -> &str {
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
pub fn guarantee_maximum_prefix_length(
    list: &[String],
    maximum_shared_prefix_length: usize,
) -> Vec<String> {
    let mut prefix_hashmap: HashMap<String, String> = HashMap::new();
    for this_word in list {
        // If this word is too short just skip it.
        if count_characters(this_word) < maximum_shared_prefix_length {
            continue;
        }
        prefix_hashmap
            .entry(get_prefix(this_word, maximum_shared_prefix_length))
            .and_modify(|existing_word| {
                // Prefer shorter words, as a stand-in for simplicity (though that
                // is debatable...)
                if count_characters(this_word) < count_characters(existing_word) {
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

/// Executes Schlinkert prune. Attempts to make list uniquely decodable
/// by removing the fewest number of code words possible. Adapted from
/// Sardinas-Patterson algorithm.
/// Runs word list both as given and with each word reversed, preferring
/// which ever preserves more words from the given list.
pub fn schlinkert_prune(list: &[String]) -> Vec<String> {
    // Clumsily clone the list into a new variable.
    let mut new_list = list.to_owned();
    // First, simply find the "offenders" with the list as given.
    let offenders_to_remove_forwards = get_sardinas_patterson_final_intersection(list);
    // Now, reverse all words before running the Schlinkert prune.
    // This will give a different list of offending words -- and potentially FEWER
    // than running the prune forwards. (We call reverse_all_words function
    // twice because we have to un-reverse all the offending words at the end.)
    let offenders_to_remove_backwards = reverse_all_words(
        &get_sardinas_patterson_final_intersection(&reverse_all_words(list)),
    );
    // If running the prune on the reversed words yielded fewer offenders
    // we'll remove those offending words, since our goal is to remove
    // the fewest number of words as possible.
    if offenders_to_remove_forwards.len() <= offenders_to_remove_backwards.len() {
        new_list.retain(|x| !offenders_to_remove_forwards.contains(x));
    } else {
        new_list.retain(|x| !offenders_to_remove_backwards.contains(x));
    }
    new_list
}

/// Reverse all words on given list. For example,
/// `["hotdog", "hamburger", "alligator"]` becomes
/// `["godtoh", "regrubmah", "rotagilla"]`
/// Uses graphemes to ensure it handles accented characters correctly.
pub fn reverse_all_words(list: &[String]) -> Vec<String> {
    let mut reversed_list = vec![];
    for word in list {
        reversed_list.push(word.graphemes(true).rev().collect::<String>());
    }
    reversed_list
}

use unicode_segmentation::UnicodeSegmentation;
/// Given a word and a `usize` of `length`, this function returns
/// the first `length` characters of that word. This length is
/// measured in grapheme clusters to better handle accented
/// characters and emoji.
/// ```
/// use tidy::list_manipulations::get_prefix;
/// assert_eq!(get_prefix("hello world", 4), "hell");
/// assert_eq!(get_prefix("sécréter", 5), "sécré");
/// assert_eq!(get_prefix("😀😃😄😁😆", 2), "😀😃");
/// ```
pub fn get_prefix(word: &str, length: usize) -> String {
    word.graphemes(true).take(length).collect::<String>()
}

/// Helper function to determine if a given char as `u16` is a
/// Latin letter (A through Z or a through z, no diacritics).
/// ```
/// use tidy::list_manipulations::is_latin_alphabetic;
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
pub fn straighten_quotes(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '\u{201C}' => result.push('\"'), // left double quotation mark
            '\u{201D}' => result.push('\"'), // right double quotation mark
            '\u{2018}' => result.push('\''), // left single quotation mark
            '\u{2019}' => result.push('\''), // right single quotation mark
            _ => result.push(c),
        }
    }
    result
}

use itertools::Itertools;
/// De-duplicates a Vector of `String`s while maintaining list order.
pub fn dedup_without_sorting(list: &mut [String]) -> Vec<String> {
    let dedup: Vec<String> = list.iter().unique().map(|s| s.to_string()).collect();
    dedup.to_vec()
}

/// Remove prefix words from the given Vector of `String`s.
///
/// A brief example: If both "news" and "newspaper" are on the inputted list
/// we may, for security reasons, want to remove the prefix word,
/// which is "news" in this case.
pub fn remove_prefix_words(list: Vec<String>) -> Vec<String> {
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
pub fn remove_suffix_words(list: Vec<String>) -> Vec<String> {
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
pub fn enfore_minimum_edit_distance(
    list: Vec<String>,
    minimum_edit_distance: usize,
) -> Vec<String> {
    let minimum_edit_distance: u32 = minimum_edit_distance.try_into().unwrap();
    let mut list_to_read = list.to_vec();
    // Sort short words first to prefer them
    list_to_read.sort_by_key(|a| count_characters(a));

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
pub fn remove_homophones(list: Vec<String>, homophones: Vec<(String, String)>) -> Vec<String> {
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
