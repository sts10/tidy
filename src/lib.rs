use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn make_vec(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            word_list.push(l);
        }
    }
    word_list
}

pub fn tidy_list(
    list: Vec<String>,
    to_lowercase: bool,
    should_remove_prefix_words: bool,
    should_remove_integers: bool,
    should_remove_through_first_tab: bool,
    reject_list: Option<Vec<String>>,
) -> Vec<String> {
    let mut tidied_list = if should_remove_through_first_tab {
        list.iter()
            .map(|w| remove_through_first_tab(w.to_string()))
            .collect()
    } else {
        list
    };
    tidied_list = if should_remove_integers {
        tidied_list
            .iter()
            .map(|w| remove_integers(w.to_string()))
            .collect()
    } else {
        tidied_list
    };
    tidied_list = trim_whitespace(&tidied_list);
    tidied_list = remove_blank_lines(&tidied_list);
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list = if to_lowercase {
        tidied_list.iter().map(|w| w.to_ascii_lowercase()).collect()
    } else {
        tidied_list
    };
    tidied_list = match reject_list {
        Some(reject_list) => remove_reject_words(tidied_list, reject_list),
        None => tidied_list,
    };
    tidied_list = if should_remove_prefix_words {
        remove_prefix_words(sort_and_dedup(&mut tidied_list))
    } else {
        tidied_list
    };
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list
}

fn remove_integers(mut w: String) -> String {
    w.retain(|c| !c.is_numeric());
    w
}

fn remove_through_first_tab(l: String) -> String {
    if l.contains("	") {
        l.split("	").collect::<Vec<&str>>()[1].to_string()
    } else {
        l
    }
}

fn remove_blank_lines(list: &[String]) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| !x.is_empty());
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
                // this is a prefix word, so we do NOT want to retain it. return false to the
                // retain
                return false;
            } else {
                // this particular word is not a prefix word of this potential_prefix_word.
                // keep looping
                continue;
            };
        }
        // if we've made it here, we can be sure that potential_prefix_word is NOT a
        // prefix word. So we want to retain it for the list_without_prefix_words.
        // return true to the retain
        true
    });
    list_without_prefix_words
}

fn remove_reject_words(list: Vec<String>, reject_list: Vec<String>) -> Vec<String> {
    let mut new_list = list.to_vec();
    new_list.retain(|x| !reject_list.contains(x));
    new_list
}
