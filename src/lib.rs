use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct TidyRequest {
    pub list: Vec<String>,
    pub to_lowercase: bool,
    pub should_remove_prefix_words: bool,
    pub should_remove_integers: bool,
    pub should_remove_through_first_tab: bool,
    pub should_remove_through_first_space: bool,
    pub reject_list: Option<Vec<String>>,
    pub approved_list: Option<Vec<String>>,
    pub minimum_length: Option<usize>,
}

pub fn make_vec_from_filenames(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error reading a line from file: {}", e);
                    continue;
                }
            };
            word_list.push(l);
        }
    }
    word_list
}

pub fn tidy_list(req: TidyRequest) -> Vec<String> {
    // guess this function is what I should clean-up next...
    let mut tidied_list = if req.should_remove_through_first_tab {
        req.list
            .iter()
            .map(|w| remove_through_first_tab(w.to_string()))
            .collect()
    } else {
        req.list
    };
    tidied_list = if req.should_remove_through_first_space {
        tidied_list
            .iter()
            .map(|w| remove_through_first_space(w.to_string()))
            .collect()
    } else {
        tidied_list
    };
    tidied_list = if req.should_remove_integers {
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
    tidied_list = if req.to_lowercase {
        tidied_list.iter().map(|w| w.to_ascii_lowercase()).collect()
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
    tidied_list = if req.should_remove_prefix_words {
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
    if l.contains('\t') {
        l.split('\t').collect::<Vec<&str>>()[1].to_string()
    } else {
        l
    }
}

fn remove_through_first_space(l: String) -> String {
    if l.contains(' ') {
        l.split(' ').collect::<Vec<&str>>()[1].to_string()
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

// pretty sure this is an accurate, if wonky method of calculating
// entropy of the list, given its size?
pub fn calc_entropy(list_size: usize) -> f64 {
    (list_size as f64).ln() / (2_f64.ln() as f64)
}
